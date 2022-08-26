pub struct FrameTimer {
    current_frame: u32,
    frames: u32,
}
impl FrameTimer {
    pub fn new(frames: u32) -> FrameTimer {
        FrameTimer { current_frame: frames, frames }
    }

    pub fn update(&mut self) -> bool {
        self.current_frame -= 1;
        if self.current_frame == 0 {
            self.current_frame = self.frames;    
            return true;
        }
        false
    }
}

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