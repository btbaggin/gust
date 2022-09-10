use super::EnemyType;
use crate::entity::EntityManager;
use crate::utils::Timer;

pub struct Wave {
    enemy_count: u32,
    spawned_count: u32,
    spawn_interval: Timer,
    enemy_type: EnemyType,
}
impl Wave {
    pub fn new(enemy_count: u32, spawn_interval: f32, enemy_type: EnemyType) -> Wave {
        Wave {
            enemy_count,
            spawned_count: 0,
            spawn_interval: Timer::new(spawn_interval),
            enemy_type,
        }
    }

    pub fn update(&mut self, delta_time: f32, entities: &mut EntityManager) {
        if self.spawned_count == self.enemy_count {
            return;
        }

        if self.spawn_interval.update(delta_time) {
            let entity = match self.enemy_type {
                EnemyType::Slime => crate::gust::enemy::Enemy::new(),
            };
            entities.create(entity);
            self.spawned_count += 1
        }
    }
}

pub struct EnemySpawner {
    waves: Vec<Wave>,
    intervals: Vec<f32>,
    time_since_start: f32,
    wave_index: usize,
    total_enemies: u32,
    enemies_killed: u32,
}
impl EnemySpawner {
    pub fn new() -> EnemySpawner {
        EnemySpawner {
            waves: vec!(),
            intervals: vec!(),
            time_since_start: 0.,
            wave_index: 0,
            total_enemies: 0,
            enemies_killed: 0,
        }
    }

    pub fn add_wave(&mut self, wave: Wave, spawn_start: f32) {
        self.total_enemies += wave.enemy_count;
        self.waves.push(wave);
        self.intervals.push(spawn_start);
    }

    pub fn mark_enemy_dead(&mut self) -> bool {
        self.enemies_killed += 1;
        self.enemies_killed == self.total_enemies

    }

    pub fn update(&mut self, delta_time: f32, entities: &mut EntityManager) {
        self.time_since_start += delta_time;
        for i in self.wave_index..self.intervals.len() {
            if self.time_since_start >= self.intervals[i] {
                self.wave_index = i;
            }
        }
        
        for i in 0..self.wave_index + 1 {
            self.waves[i].update(delta_time, entities);
        }
    }
}