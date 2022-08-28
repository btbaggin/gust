mod player;
mod enemy;
mod level;
mod level_layout;
mod tower;
pub mod main_menu;

#[repr(u8)]
pub enum PhysicsLayers {
    Enemy = 1,
    Player = 2,
}