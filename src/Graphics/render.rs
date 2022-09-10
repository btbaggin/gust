use crate::graphics::{Vertex, RenderObjectTypes, Texture, CIRCLE_FRAGMENTS};
use crate::utils::Rectangle;
use crate::V2;
use std::rc::Rc;
use crate::graphics::font::TextLayout;

use speedy2d::font::FormattedTextBlock;

impl crate::graphics::Graphics {
    // pub fn draw_triangle(&mut self) {
    //     self.push_object(RenderObjectTypes::Triangle, None, V2::new(0., 0.), V2::new(100., 100.));

    //     let vertex1 = Vertex { position: [-0.5, -0.5, 0.], tex_coords: [0.0, 0.0], color: [1., 0., 0., 1.] };
    //     let vertex2 = Vertex { position: [ 0.0,  0.5, 0.], tex_coords: [0.0, 1.0], color: [0., 1., 0., 1.] };
    //     let vertex3 = Vertex { position: [ 0.5, -0.5, 0.], tex_coords: [1.0, 0.0], color: [0., 0., 1., 1.] };
    //     self.write_vert_and_ind(&[vertex1, vertex2, vertex3], &[0, 1, 2]);
    // }

    pub fn draw_rectangle(&mut self, rect: Rectangle, color: speedy2d::color::Color) {
        self.push_object(RenderObjectTypes::Quad, None, rect.top_left(), rect.size());
        self.push_quad(V2::new(0., 0.), V2::new(1., 1.), color);
    }

    pub fn draw_image(&mut self, rect: Rectangle, image: &Texture) {
        self.push_object(RenderObjectTypes::Quad, Some(image.handle()), rect.top_left(), rect.size());
        self.push_quad(V2::new(0., 0.), V2::new(1., 1.), speedy2d::color::Color::WHITE);
    }

    pub fn draw_text(&mut self, position: V2, color: speedy2d::color::Color, text: &Rc<TextLayout>) {
        let (screen_width, screen_height) = {
            let (w, h) = self.display.get_framebuffer_dimensions();
            (w as f32, h as f32)
        };

        let mut glyph_count = 0;
        use rusttype::{point, vector, Rect};
        let origin = V2::new(position.x / screen_width, -position.y / screen_height);// V2::new(position.x / screen_width, position.y / screen_height - 0.5);
        let mut verts = vec!();
        let mut inds = vec!();
        for g in &text.glyphs {
            if let Ok(Some((uv_rect, screen_rect))) = text.cache.rect_for(0, g) {
                let rect_min = (origin + V2::new(
                                        screen_rect.min.x as f32 / screen_width - 0.5,
                                        1.0 - screen_rect.min.y as f32 / screen_height - 0.5,
                                    )) * 2.0;
                let rect_max = (origin + V2::new(
                                        screen_rect.max.x as f32 / screen_width - 0.5,
                                        1.0 - screen_rect.max.y as f32 / screen_height - 0.5,
                                    )) * 2.0;


                let uv_min = uv_rect.min;
                let uv_max = uv_rect.max;
                let vertex1 = Vertex { position: [rect_min.x, rect_min.y, 0.], tex_coords: [uv_min.x, uv_min.y], color: [color.r(), color.g(), color.b(), color.a()], };
                let vertex2 = Vertex { position: [rect_min.x, rect_max.y, 0.], tex_coords: [uv_min.x, uv_max.y], color: [color.r(), color.g(), color.b(), color.a()], };
                let vertex3 = Vertex { position: [rect_max.x, rect_max.y, 0.], tex_coords: [uv_max.x, uv_max.y], color: [color.r(), color.g(), color.b(), color.a()], };
                let vertex4 = Vertex { position: [rect_max.x, rect_min.y, 0.], tex_coords: [uv_max.x, uv_min.y], color: [color.r(), color.g(), color.b(), color.a()], };
        
                let base = (glyph_count * 4) as u16;
                verts.append(&mut vec![vertex1, vertex2, vertex3, vertex4]);
                inds.append(&mut vec![base + 0, base + 1, base + 2, base + 2, base + 3, base + 0]);

                glyph_count += 1
            }
        }
        let tex = text.cache_tex.clone();
        self.push_object_without_transform(RenderObjectTypes::Text(glyph_count), Some(tex));
        self.write_vert_and_ind(&verts, &inds);
    }

    pub fn draw_circle(&mut self, position: V2, radius: f32, color: speedy2d::color::Color) {
        const INCREMENT: f64 = 2. * std::f64::consts::PI / CIRCLE_FRAGMENTS as f64;

        self.push_object(RenderObjectTypes::Circle, None, position, V2::new(radius, radius));
        
        let mut curr_angle: f64 = 2. * std::f64::consts::PI;
        let mut verts = vec!();
        for _ in 0..CIRCLE_FRAGMENTS {
            let position = [curr_angle.cos() as f32, curr_angle.sin() as f32, 0.];
            let vertex = Vertex { position, tex_coords: [0., 0.], color: [color.r(), color.g(), color.b(), color.a()] };
            verts.push(vertex);

            curr_angle -= INCREMENT;
        }
        self.write_vert_and_ind(&verts, &[]);
    }

    pub fn draw_rectangle_image_subset(&mut self, rect: Rectangle, bounds: Rectangle, image: &Texture) {
        self.push_object(RenderObjectTypes::Quad, Some(image.handle()), rect.top_left(), rect.size());
        self.push_quad(bounds.top_left(), bounds.bottom_right(), speedy2d::color::Color::WHITE);
    }


    fn push_quad(&mut self, uv_min: V2, uv_max: V2, color: speedy2d::color::Color) {
        // let vertex1 = Vertex { position: [-0.5, -0.5, 0.], tex_coords: [0.0, 1.0], color: [1., 0., 0., 1.] };
        // let vertex2 = Vertex { position: [-0.5,  0.5, 0.], tex_coords: [0.0, 0.0], color: [0., 1., 0., 1.] };
        // let vertex3 = Vertex { position: [ 0.5,  0.5, 0.], tex_coords: [1.0, 0.0], color: [0., 0., 1., 1.] };
        // let vertex4 = Vertex { position: [ 0.5, -0.5, 0.], tex_coords: [1.0, 1.0], color: [0., 0., 1., 1.] };
        let vertex1 = Vertex { position: [0., 0., 0.], tex_coords: [uv_min.x, uv_max.y], color: [color.r(), color.g(), color.b(), color.a()] };
        let vertex2 = Vertex { position: [0.,  1.0, 0.], tex_coords: [uv_min.x, uv_min.y], color: [color.r(), color.g(), color.b(), color.a()] };
        let vertex3 = Vertex { position: [ 1.0,  1.0, 0.], tex_coords: [uv_max.x, uv_min.y], color: [color.r(), color.g(), color.b(), color.a()] };
        let vertex4 = Vertex { position: [ 1.0, 0.0, 0.], tex_coords: [uv_max.x, uv_max.y], color: [color.r(), color.g(), color.b(), color.a()] };
        self.write_vert_and_ind(&[vertex1, vertex2, vertex3, vertex4], &[0, 1, 2, 2, 3, 0]);
    }

    // fn push_text_quad(&mut self, pos: Rectangle, uv: rusttype::Rect<f32>, color: speedy2d::color::Color) -> (Vec<Vertex>, Vec<u16>) {
    //     // let vertex1 = Vertex { position: [-0.5, -0.5, 0.], tex_coords: [0.0, 1.0], color: [1., 0., 0., 1.] };
    //     // let vertex2 = Vertex { position: [-0.5,  0.5, 0.], tex_coords: [0.0, 0.0], color: [0., 1., 0., 1.] };
    //     // let vertex3 = Vertex { position: [ 0.5,  0.5, 0.], tex_coords: [1.0, 0.0], color: [0., 0., 1., 1.] };
    //     // let vertex4 = Vertex { position: [ 0.5, -0.5, 0.], tex_coords: [1.0, 1.0], color: [0., 0., 1., 1.] };     
    //     let rect_min = pos.top_left();
    //     let rect_max = pos.bottom_right();
    //     let uv_min = uv.min;
    //     let uv_max = uv.max;
    //     let vertex1 = Vertex { position: [rect_min.x, rect_min.y, 0.], tex_coords: [uv_min.x, uv_min.y], color: [color.r(), color.g(), color.b(), color.a()], };
    //     let vertex2 = Vertex { position: [rect_min.x, rect_max.y, 0.], tex_coords: [uv_min.x, uv_max.y], color: [color.r(), color.g(), color.b(), color.a()], };
    //     let vertex3 = Vertex { position: [rect_max.x, rect_max.y, 0.], tex_coords: [uv_max.x, uv_max.y], color: [color.r(), color.g(), color.b(), color.a()], };
    //     let vertex4 = Vertex { position: [rect_max.x, rect_min.y, 0.], tex_coords: [uv_max.x, uv_min.y], color: [color.r(), color.g(), color.b(), color.a()], };

    //     (vec![vertex1, vertex2, vertex3, vertex4], vec![0, 1, 2, 2, 3, 0])
    // }
}