use serde::Deserialize;
use crate::gust::enemy::EnemyType;

#[derive(Deserialize)]
pub struct Level {
    pub layout: Vec<(u32, u32)>,
    pub waves: Vec<Wave>,
    pub towers: Vec<(u32, u32)>
}
#[derive(Deserialize)]
pub struct Wave {
    pub count: u32,
    pub interval: f32,
    pub time: f32,
    pub r#type: EnemyType,
}