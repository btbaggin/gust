use std::rc::Rc;
use super::ImageHandle;
use crate::V2;
use crate::utils::Rectangle;

pub struct Texture {
    image: Rc<ImageHandle>,
    bounds: Option<Rectangle>,
}
impl Texture {
    pub fn new(image: Rc<ImageHandle>, bounds: Option<Rectangle>) -> Texture {
        Texture { image, bounds }
    }
    pub fn size(&self) -> V2 {
        let (width, height) = self.image.dimensions();
        V2::new(width as f32, height as f32)
    }
    pub fn handle(&self) -> Rc<ImageHandle> {
        self.image.clone()
    }
}
