use speedy2d::color::Color;
use crate::V2;
use crate::input::Input;
use crate::utils::from_v2;
use crate::entity::{Entity, EntityHelper};
use crate::physics::{PhysicsMaterial, Polygon, CollisionShape};

pub struct Circle { }
impl Circle {
    pub fn new() -> Circle {
        Circle { }
    }
}
impl crate::entity::EntityBehavior for Circle {
    crate::entity!();
    
    fn initialize(&self, e: &mut EntityHelper) {
        //let shape = CollisionShape::Polygon(Polygon::rectangle(75., 75., V2::new(0., 0.)));
        let shape = CollisionShape::Circle(crate::physics::Circle::new(75.));
        e.set_position(V2::new(100., 100.))
         .attach_rigid_body(PhysicsMaterial::METAL, shape)
         .set_scale(V2::new(75., 75.));
    }

    fn update(&mut self, e: &mut EntityHelper, delta_time: f32, input: &Input) { }
    fn render(&self, e: &Entity, graphics: &mut crate::Graphics) {
        //TODO add wrappers for graphics
//        crate::assets::Texture::render(graphics, crate::assets::Images::Testing, crate::utils::sized_rect(e.position, e.scale));
        graphics.draw_circle(from_v2(e.position + e.scale / 2.), e.scale.x, Color::RED);
    }
}