mod slime;
mod spawner;
pub use slime::Slime;
pub use spawner::{Wave, EnemySpawner};

pub enum EnemyType {
    Slime,
}
