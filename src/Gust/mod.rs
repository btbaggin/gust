mod player;
mod enemy;
mod level;
mod level_layout;
mod tower;
pub mod main_menu;

crate::physics_layer_enum! {
    pub enum PhysicsLayers {
        Enemy = 1,
        Player = 2,
        Bullet = 4,
    }
}