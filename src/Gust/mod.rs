mod player;
pub use player::Player;

#[derive(Eq, Hash, PartialEq)]
pub enum EntityTag {
    Player,
}