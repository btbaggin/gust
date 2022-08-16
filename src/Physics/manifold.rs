use crate::V2;
use crate::physics::RigidBody;
use crate::utils::{cross, cross_v2, scale_v2};
use cgmath::InnerSpace;

pub struct ManifoldHandle {
    pub body_a: usize,
    pub body_b: usize,
    pub manifold: Manifold,
}

pub struct Manifold {
    pub penetration: f32,
    pub normal: V2,
    pub contacts: [V2; 2],
    pub contact_count: usize,
    pub e: f32,
    pub df: f32,
    pub sf: f32
}
//https://github.com/btbaggin/Bang/blob/d38d8151bdf5f33932871daa9198ac80117ae369/Bang/Manifold.cpp
impl Manifold {
    pub fn new() -> Manifold {
        Manifold {
            penetration: 0.,
            normal: V2::new(0., 0.),
            contacts: [V2::new(0., 0.); 2],
            contact_count: 0,
            e: 0.,
            df: 0.,
            sf: 0.,
        }
    }

    pub unsafe fn initialize(&mut self, delta_time: f32, body_a: &RigidBody, body_b: &RigidBody) {
        let entity_a = &*body_a.entity;
        let entity_b = &*body_b.entity;

        // Calculate average restitution
	    self.e = f32::min(body_a.restitution, body_b.restitution);

        // Calculate static and dynamic friction
        self.sf = f32::sqrt(body_a.static_friction * body_a.static_friction);
        self.df = f32::sqrt(body_a.dynamic_friction * body_a.dynamic_friction);

        for i in 0..self.contact_count {
            // Calculate radii from COM to contact
            let ra = self.contacts[i] - entity_a.position;
            let rb = self.contacts[i] - entity_b.position;

            let rv = body_b.velocity + cross(body_b.angular_velocity, rb) -
                     body_a.velocity - cross(body_a.angular_velocity, ra);

            // Determine if we should perform a resting collision or not
            // The idea is if the only thing moving this object is gravity,
            // then the collision should be performed without any restitution
            if rv.magnitude() < scale_v2(super::GRAVITY, delta_time).magnitude() + f32::EPSILON {
                self.e = 0.;
            }
        }
    }

    pub unsafe fn apply_impulse(&self, body_a: &mut RigidBody, body_b: &mut RigidBody) {
        let entity_a = &*body_a.entity;
        let entity_b = &*body_b.entity;

        if f32::abs(body_a.inverse_mass + body_b.inverse_mass) < f32::EPSILON {
            body_a.velocity = V2::new(0., 0.);
            body_b.velocity = V2::new(0., 0.);
        }

        for i in 0..self.contact_count {
            let ra = self.contacts[i] - entity_a.position;
            let rb = self.contacts[i] - entity_b.position;

            let rv = body_b.velocity + cross(body_b.angular_velocity, rb) -
                     body_a.velocity - cross(body_a.angular_velocity, ra);

            let contact_vel = rv.dot(self.normal);
            if contact_vel > 0. { return; }

            let ra_cross_n = cross_v2(ra, self.normal);
            let rb_cross_n = cross_v2(rb, self.normal);
            let inv_mass_sum = body_a.inverse_mass + body_b.inverse_mass + (ra_cross_n * ra_cross_n) *
                               body_a.inverse_inertia + (rb_cross_n * rb_cross_n) * body_b.inverse_inertia;
            
            let j = -(1. + self.e) * contact_vel;
            let j = j / inv_mass_sum;
            let j = j / self.contact_count as f32;

            let impulse = self.normal * j;
            body_a.apply_impulse(impulse * -1., ra);
            body_b.apply_impulse(impulse, rb);

            let rv = body_b.velocity + cross(body_b.angular_velocity, rb) -
                     body_a.velocity - cross(body_a.angular_velocity, ra);

            let t = rv - (self.normal * rv.dot(self.normal));
            let t = if t.magnitude2() == 0. { V2::new(0., 0.) } else { t.normalize() };

            let jt = -rv.dot(t);
            let jt = jt / inv_mass_sum;
            let jt = jt / self.contact_count as f32;

            if f32::abs(jt) < f32::EPSILON { return; }

            let tangent_impulse = if f32::abs(jt) < j * self.sf {
                t * jt
            } else {
                t * -j * self.df
            };

            body_a.apply_impulse(tangent_impulse * -1., ra);
            body_b.apply_impulse(tangent_impulse, rb);
        }
    }

    pub unsafe fn positional_correction(&self, body_a: &mut RigidBody, body_b: &mut RigidBody) {
        let entity_a = &mut *body_a.entity;
        let entity_b = &mut *body_b.entity;

        const K_SLOP: f32 = 0.05;
        const PERCENT: f32 = 0.4;
        let scaled_normal = scale_v2(self.normal, PERCENT);
        let correction =  scale_v2(scaled_normal, f32::max(self.penetration - K_SLOP, 0.) / (body_a.inverse_mass + body_b.inverse_mass));
        entity_a.position -= scale_v2(correction, body_a.inverse_mass);
        entity_b.position += scale_v2(correction, body_b.inverse_mass);
    }
}