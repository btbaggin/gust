use glium::{glutin, Texture2d, VertexBuffer, IndexBuffer, Display, Program};
use glium::glutin::window::WindowBuilder;
use glium::glutin::event_loop::EventLoop;
use super::{Graphics, MAX_VERTS};
use crate::job_system::ThreadSafeJobQueue;

pub fn create_window(event_loop: &EventLoop<()>, builder: WindowBuilder, queue: ThreadSafeJobQueue) -> Graphics {
    let cb = glutin::ContextBuilder::new();
    let display = Display::new(builder, cb, &event_loop).unwrap();

    let blank_texture = create_blank_texture(&display);
    
    let vertices = VertexBuffer::empty_dynamic(&display, MAX_VERTS).unwrap();
    let indices = IndexBuffer::empty_dynamic(&display, glium::index::PrimitiveType::TrianglesList, MAX_VERTS).unwrap();

    let vertex_shader = std::str::from_utf8(include_bytes!("Shaders/vertex.glsl")).unwrap();
    let fragment_shader = std::str::from_utf8(include_bytes!("Shaders/fragment.glsl")).unwrap();
    let font_fragment_shader = std::str::from_utf8(include_bytes!("Shaders/font_fragment.glsl")).unwrap();

    let program = Program::from_source(&display, vertex_shader, fragment_shader, None).unwrap();
    let font_program = Program::from_source(&display, vertex_shader, font_fragment_shader, None).unwrap();

    Graphics {
        queue,
        display,
        blank_texture,
        vertices,
        indices,
        program,
        font_program,
        objects: vec!(),
        index_count: 0,
        vertex_count: 0,
        z_index: 0.
    }
}

fn create_blank_texture(display: &Display) -> Texture2d {
    use image::{RgbaImage, Rgba};
    let mut image = RgbaImage::new(1, 1);
    image.put_pixel(0, 0, Rgba([255, 255, 255, 255]));

    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    Texture2d::new(display, image).unwrap()
}