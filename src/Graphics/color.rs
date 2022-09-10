#[repr(C)]
#[derive(Copy, Clone)]
pub struct Color {
    color: [f32; 4]
}
impl Color {
    pub const WHITE: Color = Color::new(1., 1., 1., 1.);
    pub const BLACK: Color = Color::new(0., 0., 0., 1.);
    pub const RED: Color = Color::new(1., 0., 0., 1.);
    pub const GREEN: Color = Color::new(0., 1., 0., 1.);
    pub const BLUE: Color = Color::new(0., 0., 1., 1.);
    pub const YELLOW: Color = Color::new(1., 1., 0., 1.);

    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color {
            color: [r, g, b, a]
        }
    }
    pub fn new_u8(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color {
            color: [r as f32 / 255., g as f32 / 255., b as f32 / 255., a as f32 / 255.]
        }
    }
    pub fn r(&self) -> f32 { self.color[0] }
    pub fn g(&self) -> f32 { self.color[1] }
    pub fn b(&self) -> f32 { self.color[2] }
    pub fn a(&self) -> f32 { self.color[3] }
    pub fn as_ref(&self) -> [f32; 4] {
        self.color
    }
}