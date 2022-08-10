use crate::{Graphics, V2, assets::Texture};
use speedy2d::shape::Rectangle;

pub struct Animation {
    image: crate::assets::Images,
    frames: u32,
    frame_timer: crate::utils::RealTimer,
    current_frame: u32,
    frame_size: V2,
    stride: u32,
}
impl Animation {
    pub fn new(image: crate::assets::Images, frames: u32, frame_size: V2, frame_duration: f32, stride: u32) -> Animation {
        Animation { 
            image,
            frames,
            frame_timer: crate::utils::RealTimer::new(frame_duration),
            current_frame: 0,
            frame_size,
            stride
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.frame_timer.update(delta_time) {
            self.current_frame += 1
        }
    }

    pub fn render(&mut self, graphics: &mut Graphics, delta_time: f32, rect: Rectangle) {
        if let Some(image) = crate::assets::request_image(graphics, self.image) {
            let x = (self.current_frame % self.stride) as f32;
            let y = (self.current_frame % self.stride) as f32;
            let top_left = V2::new(x * self.frame_size.x, y * self.frame_size.y);
            let frame = Rectangle::new(top_left, top_left + self.frame_size);
            graphics.draw_rectangle_image_subset_tinted(rect, speedy2d::color::Color::WHITE, frame, &image);
        }
    }
}