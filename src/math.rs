use crate::V2;

pub fn from_v2<T>(a: cgmath::Vector2<T>) -> speedy2d::dimen::Vector2<T> {
    speedy2d::dimen::Vector2::new(a.x, a.y)
}

pub fn sized_rect(pos: V2, size: V2) -> speedy2d::shape::Rectangle {
    speedy2d::shape::Rectangle::new(from_v2(pos), from_v2(pos + size))
}

pub fn lerp(current: f32, target: f32, amount: f32) -> f32 {
    (1. - amount) * current + amount * target
}

//TODO method to center v2 in rect