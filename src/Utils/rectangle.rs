use crate::V2;

#[derive(Debug, PartialEq, Clone)]
pub struct Rectangle {
    top_left: V2,
    bottom_right: V2, 
}
impl Rectangle {
    pub fn new(position: V2, size: V2) -> Self {
        Rectangle {
            top_left: position,
            bottom_right: position + size
        }
    }
    pub fn from_coords(top_left: V2, bottom_right: V2) -> Self {
        Rectangle { top_left, bottom_right }
    }
    pub fn width(&self) -> f32 { self.bottom_right.x - self.top_left.x }
    pub fn height(&self) -> f32 { self.bottom_right.y - self.top_left.y }
    pub fn size(&self) -> V2 { self.bottom_right - self.top_left }
    pub fn left(&self) -> f32 { self.top_left.x }
    pub fn right(&self) -> f32 { self.bottom_right.x }
    pub fn top(&self) -> f32 { self.top_left.y }
    pub fn bottom(&self) -> f32 { self.bottom_right.y }
    pub fn top_left(&self) -> V2 { self.top_left }
    pub fn bottom_right(&self) -> V2 { self.bottom_right }
    pub fn contains(&self, point: V2) -> bool {
        point.x >= self.top_left.x
            && point.y >= self.top_left.y
            && point.x < self.bottom_right.x
            && point.y < self.bottom_right.y
    }
    pub fn intersect(&self, other: &Self) -> Option<Self> {
        let result = Self {
            top_left: V2::new(
                self.top_left.x.max(other.top_left.x),
                self.top_left.y.max(other.top_left.y)
            ),
            bottom_right: V2::new(
                self.bottom_right.x.min(other.bottom_right.x),
                self.bottom_right.y.min(other.bottom_right.y)
            )
        };

        if result.top_left.x < result.bottom_right.x &&
           result.top_left.y < result.bottom_right.y {
            Some(result)
        } else {
            None
        }
    }
}

impl std::convert::Into<speedy2d::shape::Rectangle> for Rectangle {
    fn into(self) -> speedy2d::shape::Rectangle {
        let top_left = self.top_left();
        let bottom_right = self.bottom_right();
        speedy2d::shape::Rectangle::from_tuples((top_left.x, top_left.y), (bottom_right.x, bottom_right.y))
    }
}