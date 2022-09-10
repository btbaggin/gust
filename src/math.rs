pub fn lerp(current: f32, target: f32, amount: f32) -> f32 {
    (1. - amount) * current + amount * target
}