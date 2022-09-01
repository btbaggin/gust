#![allow(dead_code)]
use crate::physics::{RigidBody, RigidBodyHandle, PhysicsMaterial, CollisionShape};
use crate::V2;

pub struct EntityInitialization<'a> {
    pub(super) position: &'a mut V2,
    pub(super) scale: &'a mut V2,
    pub(super) rotation: &'a mut f32, // radians
    pub(super) material: Option<PhysicsMaterial>,
    pub(super) shape: Option<CollisionShape>,
    pub(super) layer: u8,
    pub(super) colliding_layers: u8,
}
impl<'a> EntityInitialization<'a> {
    pub fn attach_rigid_body(&mut self, material: PhysicsMaterial, shape: CollisionShape) -> &mut EntityInitialization<'a> { 
        self.material = Some(material);
        self.shape = Some(shape);
        self
    }
    pub fn collision_layer(&mut self, layer: impl std::convert::Into<u8>) -> &mut EntityInitialization<'a> {
        self.layer = layer.into();
        self
    }
    pub fn collides_with(&mut self, layer: impl std::convert::Into<u8>) -> &mut EntityInitialization<'a> {
        self.colliding_layers |= layer.into();
        self
    }
    pub fn position(&self) -> V2 { *self.position }
    pub fn set_position(&mut self, position: V2) -> &mut EntityInitialization<'a> {
        *self.position = position;
        self
    }
    pub fn set_scale(&mut self, x: f32, y: f32) -> &mut EntityInitialization<'a> {
        *self.scale = V2::new(x, y);
        self
    }
    pub fn set_rotation(&mut self, rotation: f32) -> &mut EntityInitialization<'a> {
        *self.rotation = rotation;
        self
    }
}

pub struct EntityUpdate<'a> {
    pub(super) position: &'a mut V2,
    pub(super) scale: &'a mut V2,
    pub(super) rotation: &'a mut f32, // radians
    pub(super) rigid_body: &'a mut Option<RigidBodyHandle>,
    pub(super) mark_for_destroy: &'a mut bool,
}
impl<'a> EntityUpdate<'a> {
    pub fn position(&self) -> V2 { *self.position }
    pub fn set_position(&mut self, position: V2) -> &mut EntityUpdate<'a> {
        *self.position = position;
        self
    }
    pub fn alter_position(&mut self, delta: V2) -> &mut EntityUpdate<'a> {
        *self.position += delta;
        self
    }
    pub fn apply_force(&mut self, x: f32, y: f32) -> &mut EntityUpdate<'a> {
        if let Some(handle) = *self.rigid_body {
            RigidBody::get(handle).apply_force(V2::new(x, y));
        }
        self
    }
    pub fn set_scale(&mut self, x: f32, y: f32) -> &mut EntityUpdate<'a> {
        *self.scale = V2::new(x, y);
        self
    }
    pub fn set_rotation(&mut self, rotation: f32) -> &mut EntityUpdate<'a> {
        *self.rotation = rotation;
        if let Some(handle) = *self.rigid_body {
            RigidBody::get(handle).rotate(rotation);
        }
        self
    }
    pub fn destroy(&mut self) {
        *self.mark_for_destroy = true;
    }
}