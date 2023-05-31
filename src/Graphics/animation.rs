#![allow(dead_code)]

use std::collections::HashMap;
use std::hash::Hash;
use crate::utils::Rectangle;
use crate::{Graphics, V2, V2U, assets::Images};
use crate::utils::Timer;

pub enum SpriteSheetOrientation {
    Horizontal,
    Vertical,
}

struct Animation {
    frame_count: u32,
    index: u32,
    frame_size: V2U,
    frame_duration: f32
}

pub struct AnimationPlayer<T: Eq + Hash> {
    animations: HashMap<T, Animation>,
    current: Option<T>,
    frame_timer: Timer,
    current_frame: u32,
    image: Images,
    orientation: SpriteSheetOrientation,
}
impl<T: Eq + Hash> AnimationPlayer<T> {
    pub fn new(image: Images, orientation: SpriteSheetOrientation) -> AnimationPlayer<T> {
        AnimationPlayer {
            animations: HashMap::new(),
            current: None,
            frame_timer: Timer::new(0.),
            current_frame: 0,
            image,
            orientation
        }
    }

    pub fn add(&mut self, key: T, index: u32, frame_count: u32, frame_size: V2U, frame_duration: f32) {
        let animation = Animation { frame_count, index, frame_size, frame_duration };
        self.animations.insert(key, animation);
    }

    pub fn play(&mut self, key: T) {
        match self.animations.get(&key) {
            Some(a) => {
                self.current = Some(key);
                self.current_frame = 0;
                self.frame_timer = Timer::new(a.frame_duration);
            },
            None => panic!("Requested to pay animation that hasn't been added to player"),
        }
    }

    pub fn update(&mut self, state: &crate::UpdateState) {
        if self.frame_timer.update(state.delta_time) {
            self.current_frame += 1;
        }
    }

    pub fn render(&self, graphics: &mut Graphics, rect: Rectangle) {
        if let Some(key) = &self.current {
            if let Some(image) = crate::assets::request_image(graphics, self.image) {
                // key validated by play
                let animation = self.animations.get(key).unwrap();

                let frame = self.current_frame % animation.frame_count;
                let size = image.size();
                let (x, y) = match self.orientation {
                    SpriteSheetOrientation::Vertical => {
                        let y = (frame * animation.frame_size.y) as f32 / size.y;
                        ((animation.index * animation.frame_size.x) as f32 / size.x, y)

                    },
                    SpriteSheetOrientation::Horizontal => {
                        let x = (frame * animation.frame_size.x) as f32 / size.x;
                        (x, (animation.index * animation.frame_size.y) as f32 / size.y)
                    }
                };
                
                let sprite_size = V2::new(animation.frame_size.x as f32 / size.x, animation.frame_size.y as f32 / size.y);
                let frame = Rectangle::new(V2::new(x, y), sprite_size);
                graphics.draw_rectangle_image_subset(rect, frame, image);
            }
        }
    }
}

// macro_rules! animation_enum {
//     (enum $name:ident {
//         $($value:path,)
//     }) => {
//         enum $name {
//             $($value),
//         }

//         impl TryFrom<$name> for u32 {
//             type Error = ();
        
//             fn try_from(v: u32) -> Result<Self, Self::Error> {
//                 match v {
//                     x if x == $name::$value as i32 => Ok(SlimeAnimation::Idle),
//                     x if x == MyEnum::B as i32 => Ok(MyEnum::B),
//                     x if x == MyEnum::C as i32 => Ok(MyEnum::C),
//                     _ => Err(()),
//                 }
//             }
//         }
//     }
// }