use std::hash::Hash;
use crate::input::Input;
use crate::physics::{RigidBody, RigidBodyHandle, PhysicsMaterial, CollisionShape};
use crate::assets::{Texture, Images};
use crate::{utils::sized_rect, V2, Graphics};
use crate::job_system::ThreadSafeJobQueue;
use crate::game_loop::UpdateState;

mod generational_array;
mod scene;
mod scene_manager;
mod entity_manager;
pub use generational_array::{GenerationalArray, GenerationalIndex};
pub use self::scene::{SceneBehavior, SceneLoad};
pub use self::scene_manager::{SceneManager};
pub use entity_manager::{EntityManager};

const MAX_ENTITIES: usize = 512;

pub type EntityHandle = GenerationalIndex;

pub struct EntityHelper<'a> {
    self_pointer: *mut Entity,
    position: &'a mut V2,
    scale: &'a mut V2,
    rotation: &'a mut f32, // radians
    rigid_body: &'a mut Option<RigidBodyHandle>,
}
impl<'a> EntityHelper<'a> {
    pub fn attach_rigid_body(&mut self, material: PhysicsMaterial, shape: CollisionShape) -> &mut EntityHelper<'a> { 
        let rigid_body = RigidBody::attach(self.self_pointer, material, shape);
        *self.rigid_body = Some(rigid_body);
        self
    }
    pub fn set_position(&mut self, x: f32, y: f32) -> &mut EntityHelper<'a> {
        *self.position = V2::new(x, y);
        self
    }
    pub fn alter_position(&mut self, delta: V2) -> &mut EntityHelper<'a> {
        *self.position += delta;
        self
    }
    pub fn apply_force(&mut self, x: f32, y: f32) -> &mut EntityHelper<'a> {
        if let Some(handle) = *self.rigid_body {
            RigidBody::get(handle).apply_force(V2::new(x, y));
        }
        self
    }
    pub fn set_scale(&mut self, x: f32, y: f32) -> &mut EntityHelper<'a> {
        *self.scale = V2::new(x, y);
        self
    }
    pub fn set_rotation(&mut self, rotation: f32) -> &mut EntityHelper<'a> {
        *self.rotation = rotation;
        if let Some(handle) = *self.rigid_body {
            RigidBody::get(handle).rotate(rotation);
        }
        self
    }
}
macro_rules! create_helper {
    ($entity:ident) => {
        EntityHelper {
            self_pointer: $entity as *mut Entity,
            position: &mut $entity.position,
            scale: &mut $entity.scale,
            rotation: &mut $entity.rotation,
            rigid_body: &mut $entity.rigid_body,
        }
    };
}

pub struct Entity {
    pub position: crate::V2,
    pub scale: crate::V2,
    pub rotation: f32, // radians
    pub rigid_body: Option<RigidBodyHandle>,
    behavior: Box<dyn EntityBehavior>,
}
impl Entity {
    pub fn new(behvaior: impl EntityBehavior + 'static) -> Entity {
        Entity {
            position: V2::new(0., 0.),
            scale: V2::new(0., 0.),
            rotation: 0.,
            behavior: Box::new(behvaior),
            rigid_body: None,
        }

    }
    pub fn initialize(&mut self) {
        let mut helper = create_helper!(self);
        self.behavior.initialize(&mut helper);
    }
    pub fn update(&mut self, state: &mut UpdateState) {
        let mut helper = create_helper!(self);
        self.behavior.update(&mut helper, state)
    }
}

pub trait EntityBehavior: crate::messages::MessageHandler {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;

    fn initialize(&self, e: &mut EntityHelper);

    fn update(&mut self, e: &mut EntityHelper, update_state: &mut UpdateState);
    fn render(&self, e: &Entity, graphics: &mut Graphics);

    fn render_texture(&self, image: Images, e: &Entity, graphics: &mut Graphics) {
        Texture::render(graphics, image, sized_rect(e.position, e.scale));
    }
}

#[macro_export]
macro_rules! entity {
    () => {
        fn as_any(&self) -> &dyn std::any::Any { self }
        fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    };
}
