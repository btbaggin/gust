use crate::V2;
use crate::entity::Entity;

pub struct PhysicsMaterial {
    pub(super) static_friction: f32,
    pub(super) dynamic_friction: f32,
    pub(super) restitution: f32,
    pub(super) density: f32,
}
impl PhysicsMaterial {
    pub const METAL: PhysicsMaterial = PhysicsMaterial::new(0.2, 0.1, 0.1, 0.8);

    pub const fn new(static_friction: f32, dynamic_friction: f32, restitution: f32, density: f32) -> PhysicsMaterial {
        PhysicsMaterial {
            static_friction, dynamic_friction, restitution, density
        }
    }
}

pub type RigidBodyHandle = crate::generational_array::GenerationalIndex;
pub struct RigidBody {
    pub(super) entity: *mut Entity,
    
    pub(super) layer: u8,
    pub(super) colliding_layers: u8,
    pub(super) velocity: V2,
    pub(super) angular_velocity: f32,
    pub(super) torque: f32,

    pub(super) force: V2,

    inertia: f32,
    mass: f32,
    pub(super) inverse_inertia: f32,
    pub(super) inverse_mass: f32,

    pub(super) static_friction: f32,
    pub(super) dynamic_friction: f32,
    pub(super) restitution: f32,
    pub(super) shape: super::CollisionShape,
}
impl RigidBody {
    pub fn attach(entity: *mut Entity, material: PhysicsMaterial, shape: super::CollisionShape, layer: u8, colliding_layers: u8) -> RigidBodyHandle {
        let (mass, inertia) = match &shape {
            super::CollisionShape::Circle(c) => {
                let m = std::f64::consts::PI as f32 * c.radius() * c.radius() * material.density;
                let i = m * c.radius() * c.radius();
                (m, i)
            }
            super::CollisionShape::Polygon(p) => {
                p.calulate_mass_inertia(material.density)
            }
        };

        let body = RigidBody {
            entity,
            layer,
            colliding_layers,
            velocity: V2::new(0., 0.),
            angular_velocity: 0.,
            torque: 0.,
            force: V2::new(0., 0.),
            inertia,
            inverse_inertia: if inertia == 0. { 0. } else { 1. / inertia },
            mass,
            inverse_mass: if mass == 0. { 0. } else { 1. / mass },
            static_friction: material.static_friction,
            dynamic_friction: material.dynamic_friction,
            restitution: material.restitution,
            shape,
        };
        let physics = super::physics();
        physics.bodies.push(body).0
    }

    pub fn apply_impulse(&mut self, impulse: V2, contact: V2) {
        self.velocity += impulse * self.inverse_mass;
        self.angular_velocity += self.inverse_inertia * super::cross_v2(contact, impulse);
    }

    pub fn rotate(&mut self, rotation: f32) {
        self.shape.set_orient(rotation)
    }

    pub fn apply_force(&mut self, force: V2) {
        self.force += force;
    }

    pub unsafe fn notify_collision(&mut self, other: &RigidBody, messages: &mut crate::messages::MessageBus) {
        let entity = &mut *self.entity;
        let other = &*other.entity;
        entity.notify_collision(other, messages)
    }

    pub fn destroy(handle: RigidBodyHandle) {
        let physics = super::physics();
        physics.bodies.remove(&handle);
    }

    pub fn get(handle: RigidBodyHandle) -> &'static mut RigidBody {
        let physics = super::physics();
        physics.bodies.get_mut(&handle).unwrap()
    }
}

