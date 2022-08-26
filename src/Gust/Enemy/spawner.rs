use super::EnemyType;
use crate::utils::Timer;
use crate::messages::{SharedMessageBus, MessageKind};

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

    pub fn update(&mut self, delta_time: f32, messages: SharedMessageBus) {
        if self.spawned_count == self.enemy_count {
            return;
        }

        if self.spawn_interval.update(delta_time) {
            // TODO enemy type
            let mut messages = messages.borrow_mut();
            messages.send_to_group(MessageKind::SpawnEnemy, crate::get_address!(crate::gust::level::Level));
            //let manager = crate::entity::entity_manager();
            //manager.create(super::Slime::new());
            self.spawned_count += 1
        }
    }
}

pub struct EnemySpawner {
    waves: Vec<Wave>,
    intervals: Vec<f32>,
    time_since_start: f32,
    wave_index: usize,
    messages: SharedMessageBus
}
impl EnemySpawner {
    pub fn new(messages: SharedMessageBus) -> EnemySpawner {
        EnemySpawner {
            waves: vec!(),
            intervals: vec!(),
            time_since_start: 0.,
            wave_index: 0,
            messages
        }
    }

    pub fn add_wave(&mut self, wave: Wave, spawn_start: f32) {
        self.waves.push(wave);
        self.intervals.push(spawn_start);
    }

    pub fn update(&mut self, delta_time: f32) {
        self.time_since_start += delta_time;
        for i in self.wave_index..self.intervals.len() {
            if self.time_since_start >= self.intervals[i] {
                self.wave_index = i;
            }
        }
        
        for i in 0..self.wave_index + 1 {
            self.waves[i].update(delta_time, self.messages.clone());
        }
    }
}