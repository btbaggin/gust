pub struct Timer {
    current_time: f32,
    time: f32,
}
impl Timer {
    pub fn new(time: f32) -> Timer {
        Timer { current_time: time, time }
    }

    pub fn update(&mut self, delta_time: f32) -> bool {
        self.current_time -= delta_time;
        if self.current_time <= 0. {
            self.current_time += self.time;
            return true;
        }
        false
    }
}