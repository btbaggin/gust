#![allow(dead_code)]

use crate::V2;
use crate::physics::collision_shape::Shape;

mod rigid_body;
mod collision_shape;
mod manifold;
mod collision;
pub use collision_shape::{CollisionShape, Circle, Polygon};
pub use rigid_body::{PhysicsMaterial, RigidBody, RigidBodyHandle};
use manifold::{Manifold, ManifoldHandle};

static mut PHYSICS: Option<Physics> = None;

pub const GRAVITY: V2 = V2::new(0., 0.);
pub const PHYSICS_ITERATIONS: u8 = 10;

pub struct Physics {
    bodies: Vec<RigidBody>
}


pub fn initialize_physics() {
    unsafe { PHYSICS = Some(Physics { bodies: vec!() }) };
}

unsafe fn solve_manifold(body_a: &RigidBody, body_b: &RigidBody) -> Manifold {
    let mut m = Manifold::new();
    let entity_a = &*body_a.entity;
    let entity_b = &*body_b.entity;
    let p = body_a.entity as u64;
    let p2 = body_b.entity as u64;
    match (&body_a.shape, &body_b.shape) {
        (CollisionShape::Circle(a), CollisionShape::Circle(b)) => collision::circle_to_circle(&mut m, entity_a, entity_b, a, b),
        (CollisionShape::Polygon(b), CollisionShape::Circle(a)) => collision::circle_to_polygon(&mut m, entity_a, entity_b, a, b),
        (CollisionShape::Circle(a), CollisionShape::Polygon(b)) => collision::circle_to_polygon(&mut m, entity_b, entity_a, a, b),
        (CollisionShape::Polygon(a), CollisionShape::Polygon(b)) => collision::polygon_to_polygon(&mut m, entity_a, entity_b, a, b),
        _ => panic!("Unknown collision mesh combination"),
    }
    m
}

// see http://www.niksula.hut.fi/~hkankaan/Homepages/gravity.html
fn integrate_forces(body: &mut RigidBody, delta_time: f32) {
	if body.inverse_mass == 0. {
		return;
    }

	body.velocity += (body.force * body.inverse_mass + GRAVITY) * (delta_time / 2.);
	body.angular_velocity += body.torque * body.inverse_inertia * (delta_time / 2.);
}

unsafe fn integrate_velocity(body: &mut RigidBody, delta_time: f32) {
	if body.inverse_mass == 0. {
		return;
    }

    let entity = &mut *body.entity;
	entity.position += body.velocity * delta_time;
	body.orient += body.angular_velocity * delta_time;
    match &mut body.shape {
        CollisionShape::Circle(c) => c.set_orient(body.orient),
        CollisionShape::Polygon(p) => p.set_orient(body.orient),
    }
	integrate_forces(body, delta_time);
}

pub unsafe fn step_physics(delta_time: f32) {
    let physics = PHYSICS.as_mut().unwrap();
    let bodies = &mut physics.bodies;

	// Generate new collision info
	let mut contacts = vec!();
    let mut i = 0;
    while i < bodies.len() {
        let a = &bodies[i];

        let mut j = i + 1;
        while j < bodies.len() {
            let b = &bodies[j];

            if a.inverse_mass + b.inverse_mass == 0. {
                continue;
            }

            let manifold = solve_manifold(a, b);
            if manifold.contact_count > 0 {
                let handle = ManifoldHandle {
                    body_a: i,
                    body_b: j,
                    manifold,
                };
                contacts.push(handle);
            }
            j += 1;
        }
        i += 1;
    }

	// Integrate forces
	for b in bodies.iter_mut() {
		integrate_forces(b, delta_time);
    }

	// Initialize collision
	for c in &mut contacts {
        let a = &bodies[c.body_a];
        let b = &bodies[c.body_b];
		c.manifold.initialize(delta_time, a, b);
    }

	// Solve collisions
	for i in 0..PHYSICS_ITERATIONS {
		for c in &contacts {
            let a = &mut *(bodies.get_unchecked_mut(c.body_a) as *mut _);
            let b = &mut *(bodies.get_unchecked_mut(c.body_b) as *mut _);
            c.manifold.apply_impulse(a, b);
        }
    }

	// Integrate velocities
    for b in bodies.iter_mut() {
        integrate_velocity(b, delta_time);
    }

	// Correct positions
    for c in &contacts {
        let a = &mut *(bodies.get_unchecked_mut(c.body_a) as *mut _);
        let b = &mut *(bodies.get_unchecked_mut(c.body_b) as *mut _);
        c.manifold.positional_correction(a, b);
    }

	// Clear all forces
    for b in bodies.iter_mut() {
		b.force = V2::new(0., 0.);
		b.torque = 0.;
	}
}