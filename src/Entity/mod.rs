use crate::physics::{RigidBody, RigidBodyHandle, PhysicsMaterial, CollisionShape};
use crate::assets::{Texture, Images};
use crate::{math::sized_rect, V2, Graphics, UpdateState};

mod generational_array;
mod scene;
mod manager;
mod entity_helper;
pub use generational_array::{GenerationalArray, GenerationalIndex};
pub use self::scene::{SceneBehavior, SceneLoad, Scene};
pub use manager::{EntityManager, entity_manager, EntityCreationOptions};
pub use entity_helper::{EntityInitialization, EntityUpdate};

const MAX_ENTITIES: usize = 512;

pub type EntityId = std::any::TypeId;
pub type EntityHandle = GenerationalIndex;

macro_rules! create_helper {
    ($entity:ident) => {

    };
}

pub struct Entity {
    pub position: crate::V2,
    pub scale: crate::V2,
    pub rotation: f32, // radians
    pub rigid_body: Option<RigidBodyHandle>,
    mark_for_destroy: bool,
    behavior: Box<dyn EntityBehavior>,
}
impl Entity {
    pub fn new(behvaior: impl EntityBehavior + 'static) -> Entity {
        Entity {
            position: V2::new(0., 0.),
            scale: V2::new(0., 0.),
            rotation: 0.,
            behavior: Box::new(behvaior),
            mark_for_destroy: false,
            rigid_body: None,
        }

    }
    pub fn initialize(&mut self) {
        let self_pointer = self as *mut Entity;
        let mut helper = EntityInitialization {
            position: &mut self.position,
            scale: &mut self.scale,
            rotation: &mut self.rotation,
            material: None,
            shape: None,
            layer: None,
            colliding_layers: None,
        };
        self.behavior.initialize(&mut helper);
        
        if let Some(material) = helper.material {
            let layer = helper.layer.or(Some(1)).unwrap();
            let colliding_layers = helper.colliding_layers.or(Some(1)).unwrap();
            let rigid_body = RigidBody::attach(self_pointer, material, helper.shape.unwrap(), layer, colliding_layers);
            self.rigid_body = Some(rigid_body);
        }
    }
    pub fn update(&mut self, state: &mut UpdateState) {
        let mut helper = EntityUpdate {
            position: &mut self.position,
            scale: &mut self.scale,
            rotation: &mut self.rotation,
            rigid_body: &mut self.rigid_body,
            mark_for_destroy: &mut self.mark_for_destroy
        };
        self.behavior.update(&mut helper, state)
    }
    pub fn as_any(&self) -> &dyn std::any::Any {
        self.behavior.as_any()
    }
    pub fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self.behavior.as_any_mut()
    }
    pub fn destroy(&mut self) {
        self.mark_for_destroy = true;
    }
}

pub trait EntityBehavior: crate::messages::MessageHandler {
    fn id(&self) -> EntityId;
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;

    fn initialize(&mut self, e: &mut EntityInitialization);

    fn update(&mut self, e: &mut EntityUpdate, update_state: &mut UpdateState);
    fn render(&self, e: &Entity, graphics: &mut Graphics);

    fn render_texture(&self, image: Images, e: &Entity, graphics: &mut Graphics) {
        Texture::render(graphics, image, sized_rect(e.position, e.scale));
    }
}

#[macro_export]
macro_rules! entity {
    ($ty:ty) => {
        fn id(&self) -> crate::entity::EntityId { std::any::TypeId::of::<$ty>() }
        fn as_any(&self) -> &dyn std::any::Any { self }
        fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    };
}
