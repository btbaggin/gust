use glium::{Surface, implement_vertex, uniform, VertexBuffer, IndexBuffer, Display, Program};
use glium::texture::Texture2d;
use std::rc::Rc;
use cgmath::Matrix4;
use crate::V2;
use crate::utils::Rectangle;
use crate::job_system::ThreadSafeJobQueue;
use cgmath::SquareMatrix;

mod animation;
mod game_window;
mod render;
mod texture;
mod font;
mod color;
pub use animation::{AnimationPlayer, SpriteSheetOrientation};
pub use game_window::create_window;
pub use texture::Texture;
pub use font::{Font, TextLayout};
pub use color::Color;

pub type ImageHandle = Texture2d;

const MAX_VERTS: usize = 16384;
const CIRCLE_FRAGMENTS: usize = 100;

enum RenderObjectTypes {
    Quad,
    Circle,
    Text(usize),
}

struct RenderObject {
    object_type: RenderObjectTypes,
    vert_index: usize,
    ind_index: usize,
    
    matrix: Matrix4<f32>,
    image: Option<Rc<Texture2d>>
}
fn get_index_slice<'a, T: glium::index::Index>(o: &RenderObject, buffer: &'a IndexBuffer<T>, count: usize) -> glium::index::IndexBufferSlice<'a, T> {
    buffer.slice(o.ind_index..(o.ind_index + count)).unwrap()
}
fn get_vertex_slice<'a, T: Copy>(o: &RenderObject, buffer: &'a VertexBuffer<T>, count: usize) -> glium::vertex::VertexBufferSlice<'a, T> {
    buffer.slice(o.vert_index..(o.vert_index + count)).unwrap()
}

#[derive(Copy, Clone)]
struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub color: [f32; 4]
}
implement_vertex!(Vertex, position, tex_coords, color);

pub struct Graphics {
    pub queue: ThreadSafeJobQueue,
    display: Display,
    program: Program,
    font_program: Program,
    blank_texture: Texture2d,

    vertices: VertexBuffer<Vertex>,
    indices: IndexBuffer<u16>,

    objects: Vec<RenderObject>,
    vertex_count: usize,
    index_count: usize,
    z_index: f32,
}
impl Graphics {
    pub fn load_image(&self, image: glium::texture::RawImage2d<u8>) -> ImageHandle {
        ImageHandle::new(&self.display, image).unwrap()
    }

    pub fn draw_frame(&mut self) {
        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        let size = crate::game_loop::global_state().screen_size;
        let camera = cgmath::ortho(0., size.x, size.y, 0., 0., 100.);

        for o in &self.objects {
            let texture = match &o.image {
                Some(i) => i,
                None => &self.blank_texture,
            };

            let uniforms = uniform! {
                vp: Self::mat_to_array(&camera),
                matrix: Self::mat_to_array(&o.matrix),
                tex: texture
            };

            let parameters = glium::DrawParameters {
                blend: glium::Blend::alpha_blending(),
                ..Default::default()
            };

            match o.object_type {
                // RenderObjectTypes::Triangle => {
                //     let verts = get_vertex_slice(&o, &self.vertices, 3);
                //     let inds = get_index_slice(&o, &self.indices, 3);
                //     target.draw(verts, &inds, &self.program, &uniforms, &Default::default()).unwrap();
                // }
                RenderObjectTypes::Quad => {
                    let verts = get_vertex_slice(&o, &self.vertices, 4);
                    let inds = get_index_slice(&o, &self.indices, 6);
                    target.draw(verts, &inds, &self.program, &uniforms, &parameters).unwrap();
                }
                RenderObjectTypes::Circle => {
                    let verts = get_vertex_slice(&o, &self.vertices, CIRCLE_FRAGMENTS);
                    target.draw(verts, 
                                glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan),
                                &self.program,
                                &uniforms,
                                &parameters
                            ).unwrap();
                },
                RenderObjectTypes::Text(quad_count) => {
                    let uniforms = uniform! {
                        vp: Self::mat_to_array(&Matrix4::identity()),
                        matrix: Self::mat_to_array(&o.matrix),
                        tex: texture.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
                    };

                    let verts = get_vertex_slice(&o, &self.vertices, 4 * quad_count);
                    let inds = get_index_slice(&o, &self.indices, 6 * quad_count);
                    target.draw(verts, 
                        &inds,
                        &self.font_program,
                        &uniforms,
                        &parameters
                    ).unwrap();
                }
            };
        }
      
        target.finish().unwrap();
        self.index_count = 0;
        self.vertex_count = 0;
        self.objects.clear();
        self.z_index = 0.;
    }

    fn mat_to_array(matrix: &Matrix4<f32>) -> [[f32; 4]; 4] {
        *matrix.as_ref()
    }

    fn write_vert_and_ind(&mut self, verts: &[Vertex], inds: &[u16]) {
        let vertices = self.vertices.slice_mut(self.vertex_count..(self.vertex_count + verts.len()))
            .expect("Vertex count has exceeded max vertex count");
        vertices.write(&verts);
        self.vertex_count += verts.len();

        if inds.len() > 0 {
            let indices = self.indices.slice_mut(self.index_count..(self.index_count + inds.len()))
                .expect("Index count has exceed max index count");
            indices.write(&inds);
            self.index_count += inds.len();
        }
    }

    fn push_object(&mut self, object_type: RenderObjectTypes, image: Option<Rc<Texture2d>>, pos: V2, size: V2) {
        let pos = cgmath::Vector3::new(pos.x, pos.y, 0.);

        let o = RenderObject {
            object_type,
            vert_index: self.vertex_count,
            ind_index: self.index_count,
            matrix: Matrix4::from_translation(pos) * Matrix4::from_nonuniform_scale(size.x, size.y, 0.),
            image,
        };
        self.objects.push(o)
    }

    fn push_object_without_transform(&mut self, object_type: RenderObjectTypes, image: Option<Rc<Texture2d>>) {
        let o = RenderObject {
            object_type,
            vert_index: self.vertex_count,
            ind_index: self.index_count,
            matrix: Matrix4::identity(),
            image,
        };
        self.objects.push(o)
    }

    pub fn resize(&mut self, new_size: glium::glutin::dpi::PhysicalSize<u32>) {
        self.display.gl_window().resize(new_size);
        //TODO window.set_viewport_size_pixels(speedy2d::dimen::Vector2::new(physical_size.width, physical_size.height));
    }

    pub fn window_size(&self) -> V2 {
        let size = self.display.gl_window().window().inner_size();
        crate::V2::new(size.width as f32, size.height as f32)
    }

    pub fn z_index(&mut self, z_index: f32) {
        self.z_index = z_index;
    }
}


pub fn screen_rect() -> Rectangle {
    let state = crate::game_loop::global_state();
    Rectangle::new(crate::V2::new(0., 0.), state.screen_size)
}

pub fn on_screen(bounds: &Rectangle) -> bool {
    let rect = screen_rect();
    rect.intersect(&bounds).is_some()
}