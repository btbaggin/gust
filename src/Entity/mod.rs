use crate::physics::{RigidBody, RigidBodyHandle};
use crate::assets::Images;
use crate::{utils::Rectangle, V2, Graphics, UpdateState};

mod scene;
mod manager;
mod entity_helper;
pub use self::scene::{SceneBehavior, SceneLoad, Scene};
pub use manager::{EntityManager, entity_manager, EntityCreationOptions};
pub use entity_helper::{EntityInitialization, EntityUpdate};

pub const MAX_ENTITIES: usize = 512;

pub type EntityId = std::any::TypeId;
pub type EntityHandle = crate::generational_array::GenerationalIndex;

pub struct Entity {
    pub position: crate::V2,
    pub scale: crate::V2,
    pub rotation: f32, // radians
    pub rigid_body: Option<RigidBodyHandle>,
    mark_for_destroy: bool,
    behavior: Box<dyn EntityBehavior>,
}
impl Entity {
    pub fn new(behvaior: impl EntityBehavior + 'static, position: V2) -> Entity {
        Entity {
            position,
            scale: V2::new(0., 0.),
            rotation: 0.,
            behavior: Box::new(behvaior),
            mark_for_destroy: false,
            rigid_body: None,
        }

    }
    pub(super) fn initialize(&mut self) {
        let self_pointer = self as *mut Entity;
        let mut helper = EntityInitialization {
            position: &mut self.position,
            scale: &mut self.scale,
            rotation: &mut self.rotation,
            material: None,
            shape: None,
            layer: 1,
            colliding_layers: 1,
        };
        self.behavior.initialize(&mut helper);
        
        if let Some(material) = helper.material {
            let layer = helper.layer;
            let colliding_layers = helper.colliding_layers;
            let rigid_body = RigidBody::attach(self_pointer, material, helper.shape.unwrap(), layer, colliding_layers);
            self.rigid_body = Some(rigid_body);
        }
    }
    pub(super) fn update(&mut self, state: &mut UpdateState) {
        let mut helper = EntityUpdate {
            position: &mut self.position,
            scale: &mut self.scale,
            rotation: &mut self.rotation,
            rigid_body: &mut self.rigid_body,
            mark_for_destroy: &mut self.mark_for_destroy
        };
        self.behavior.update(&mut helper, state)
    }
    pub(super) fn notify_collision(&mut self, other: &Entity, messages: &mut crate::messages::MessageBus) {
        let mut helper = EntityUpdate {
            position: &mut self.position,
            scale: &mut self.scale,
            rotation: &mut self.rotation,
            rigid_body: &mut self.rigid_body,
            mark_for_destroy: &mut self.mark_for_destroy
        };
        self.behavior.on_collision(&mut helper, other, messages);
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
    pub fn bounds(&self) -> crate::utils::Rectangle {
        crate::utils::Rectangle::new(self.position, self.scale)
    }
}

pub trait EntityBehavior: crate::messages::MessageHandler {
    fn id(&self) -> EntityId;
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;

    fn initialize(&mut self, e: &mut EntityInitialization);

    fn update(&mut self, e: &mut EntityUpdate, update_state: &mut UpdateState);
    fn render(&self, e: &Entity, graphics: &mut Graphics);

    fn on_collision(&mut self, _e: &mut EntityUpdate, _other: &Entity, _messages: &mut crate::messages::MessageBus) { }

    fn render_texture(&self, image: Images, e: &Entity, graphics: &mut Graphics) {
        if let Some(i) = crate::assets::request_image(graphics, image) {
            graphics.draw_image(Rectangle::new(e.position, e.scale), i);
        }
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