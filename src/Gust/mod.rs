mod player;
mod enemy;
mod level;
mod tower;
mod cards;
pub mod main_menu;
pub mod game_data;

crate::physics_layer_enum! {
    pub enum PhysicsLayers {
        Enemy = 1,
        Player = 2,
        Bullet = 4,
    }
}