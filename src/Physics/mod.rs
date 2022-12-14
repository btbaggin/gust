#![allow(dead_code)]

use crate::V2;

mod rigid_body;
mod collision_shape;
mod manifold;
mod collision;
mod quad_tree;
pub use quad_tree::QuadTree;
pub use collision_shape::{CollisionShape, Circle, Polygon};
pub use rigid_body::{PhysicsMaterial, RigidBody, RigidBodyHandle};
use manifold::{Manifold, ManifoldHandle};
use crate::generational_array::GenerationalArray;
use crate::entity::MAX_ENTITIES;

crate::singleton!(physics: Physics = Physics { bodies: GenerationalArray::new(), gravity: V2::new(0., 0.) });
pub const PHYSICS_ITERATIONS: u8 = 10;

#[macro_export]
macro_rules! physics_layer_enum {
    ($vis:vis enum $name:ident {
        $($key:ident = $value:expr,)+
    }) => {
        #[repr(u8)]
        $vis enum $name {
            $($key = $value,)+
        }
        impl std::convert::Into<u8> for $name {
            fn into(self) -> u8 {
                self as u8
            }
        }
    }
}

pub struct Physics {
    bodies: GenerationalArray<RigidBody, MAX_ENTITIES>,
    gravity: V2,
}

pub fn cross_v2(a: V2, b: V2) -> f32 {
	a.x * b.y - a.y * b.x
}

fn cross(a: f32, b: V2) -> V2 {
    V2::new(-a * b.y, a * b.x)
}


unsafe fn solve_manifold(body_a: &RigidBody, body_b: &RigidBody) -> Manifold {
    let mut m = Manifold::new();
    let entity_a = &*body_a.entity;
    let entity_b = &*body_b.entity;

    match (&body_a.shape, &body_b.shape) {
        (CollisionShape::Circle(a), CollisionShape::Circle(b)) => collision::circle_to_circle(&mut m, entity_a, entity_b, a, b),
        (CollisionShape::Polygon(b), CollisionShape::Circle(a)) => collision::circle_to_polygon(&mut m, entity_a, entity_b, a, b),
        (CollisionShape::Circle(a), CollisionShape::Polygon(b)) => collision::circle_to_polygon(&mut m, entity_b, entity_a, a, b),
        (CollisionShape::Polygon(a), CollisionShape::Polygon(b)) => collision::polygon_to_polygon(&mut m, entity_a, entity_b, a, b),
    }
    m
}

// see http://www.niksula.hut.fi/~hkankaan/Homepages/gravity.html
fn integrate_forces(body: &mut RigidBody, gravity: V2, delta_time: f32) {
	if body.inverse_mass == 0. {
		return;
    }

	body.velocity += (body.force * body.inverse_mass + gravity) * (delta_time / 2.);
	body.angular_velocity += body.torque * body.inverse_inertia * (delta_time / 2.);
}

unsafe fn integrate_velocity(body: &mut RigidBody, gravity: V2, delta_time: f32) {
	if body.inverse_mass == 0. {
		return;
    }

    let entity = &mut *body.entity;
	entity.position += body.velocity * delta_time;
	entity.rotation += body.angular_velocity * delta_time;
    body.shape.set_orient(entity.rotation);
	integrate_forces(body, gravity, delta_time);
}

pub unsafe fn step_physics(delta_time: f32, messages: &mut crate::messages::MessageBus) {
    let physics = physics();
    let bodies = &mut physics.bodies;

	// Generate new collision info
	let mut contacts = vec!();
    let mut i = 0;
    while i < bodies.len() {
        if let Some(a) = bodies.get_at(i) {

            let mut j = i + 1;
            while j < bodies.len() {
                if let Some(b) = bodies.get_at(j) {

                    if a.inverse_mass + b.inverse_mass == 0. || 
                       a.colliding_layers & b.layer == 0 {
                        // both objects are static, no collision will occur
                        // objects will not collide due to layers
                    } else {
                        // objects will collide
                        let manifold = solve_manifold(a, b);
                        if manifold.contact_count > 0 {
                            let handle = ManifoldHandle {
                                body_a: i,
                                body_b: j,
                                manifold,
                            };
                            contacts.push(handle);
                        }
                    }
                }

                j += 1;
            }
        }
        i += 1;
    }

	// Integrate forces
	for b in bodies.iter_index() {
        let b = bodies.get_mut(&b).unwrap();
		integrate_forces(b, physics.gravity, delta_time);
    }

	// Initialize collision
	for c in &mut contacts {
        let a = &bodies.get_at(c.body_a).unwrap();
        let b = &bodies.get_at(c.body_b).unwrap();
		c.manifold.initialize(delta_time, physics.gravity, a, b);
    }

	// Solve collisions
	for _ in 0..PHYSICS_ITERATIONS {
		for c in &contacts {
            let a = &mut *(bodies.get_at_mut(c.body_a).unwrap() as *mut _);
            let b = &mut *(bodies.get_at_mut(c.body_b).unwrap() as *mut _);
            c.manifold.apply_impulse(a, b);
        }
    }

	// Integrate velocities
    for b in bodies.iter_index() {
        let b = bodies.get_mut(&b).unwrap();
        integrate_velocity(b, physics.gravity, delta_time);
    }

	// Correct positions
    for c in &contacts {
        let a = &mut *(bodies.get_at_mut(c.body_a).unwrap() as *mut _);
        let b = &mut *(bodies.get_at_mut(c.body_b).unwrap() as *mut _);
        c.manifold.positional_correction(a, b);
    }

    //Notify entities of collision
    for c in &contacts {
        let a: &mut RigidBody = &mut *(bodies.get_at_mut(c.body_a).unwrap() as *mut _);
        let b: &mut RigidBody = &mut *(bodies.get_at_mut(c.body_b).unwrap() as *mut _);
        a.notify_collision(b, messages);
        b.notify_collision(a, messages);
    }

	// Clear all forces
    for b in bodies.iter_index() {
        let mut b = bodies.get_mut(&b).unwrap();
		b.force = V2::new(0., 0.);
		b.torque = 0.;
	}
}