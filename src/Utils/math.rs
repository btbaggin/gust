use crate::V2;

pub fn cross(a: f32, b: V2) -> V2 {
    V2::new(-a * b.y, a * b.x)
}

pub fn cross_v2(a: V2, b: V2) -> f32 {
	a.x * b.y - a.y * b.x
}

pub fn scale_v2(a: V2, scale: f32) -> V2 {
    V2::new(a.x * scale, a.y * scale)
}

pub fn from_v2<T>(a: cgmath::Vector2<T>) -> speedy2d::dimen::Vector2<T> {
    speedy2d::dimen::Vector2::new(a.x, a.y)
}

pub fn sized_rect(pos: V2, size: V2) -> speedy2d::shape::Rectangle {
    speedy2d::shape::Rectangle::new(from_v2(pos), from_v2(pos + size))
}