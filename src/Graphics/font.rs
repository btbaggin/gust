use glium::*;
use glium::glutin::{
    event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
};
use rusttype::gpu_cache::Cache;
use rusttype::{point, vector, PositionedGlyph, Rect, Scale};
use std::borrow::Cow;
use std::env;
use std::error::Error;
use std::rc::Rc;
use super::Graphics;
use crate::V2;
use crate::utils::Rectangle;
use crate::logger::PanicLogEntry;

//https://gitlab.redox-os.org/redox-os/rusttype/-/blob/master/dev/examples/gpu_cache.rs
pub struct Font {
    font: rusttype::Font<'static>,
}
impl Font {
    pub fn new(font: std::path::PathBuf) -> Font {
        let data = std::fs::read(font).log_and_panic();
        let font = rusttype::Font::try_from_vec(data).unwrap();
    
        Font { font, }
    }

    pub fn layout_text(&self, graphics: &Graphics, text: &str, size: f32) -> Rc<TextLayout> {
        let scale = graphics.display.gl_window().window().scale_factor();

        let (cache_width, cache_height) = ((512.0 * scale) as u32, (512.0 * scale) as u32);
        let mut cache: Cache<'static> = Cache::builder()
            .dimensions(cache_width, cache_height)
            .build();

        let cache_tex = glium::texture::Texture2d::with_format(
            &graphics.display,
            glium::texture::RawImage2d {
                data: Cow::Owned(vec![128u8; cache_width as usize * cache_height as usize]),
                width: cache_width,
                height: cache_height,
                format: glium::texture::ClientFormat::U8,
            },
            glium::texture::UncompressedFloatFormat::U8,
            glium::texture::MipmapsOption::NoMipmap,
        ).unwrap();

        let (size, glyphs) = layout_paragraph(&self.font, Scale::uniform(size * scale as f32), i32::MAX, &text);
        for glyph in &glyphs {
            cache.queue_glyph(0, glyph.clone());
        }
        cache.cache_queued(|rect, data| {
            cache_tex.main_level().write(
                glium::Rect {
                    left: rect.min.x,
                    bottom: rect.min.y,
                    width: rect.width(),
                    height: rect.height(),
                },
                glium::texture::RawImage2d {
                    data: Cow::Borrowed(data),
                    width: rect.width(),
                    height: rect.height(),
                    format: glium::texture::ClientFormat::U8,
                },
            );
        }).unwrap();

        Rc::new(TextLayout {
            cache_tex: Rc::new(cache_tex), glyphs, cache, size
        })
    }
}

pub struct TextLayout {
    pub(super) cache_tex: Rc<glium::texture::Texture2d>,
    pub(super) glyphs: Vec<PositionedGlyph<'static>>,
    pub(super) cache: Cache<'static>,
    size: V2,
}
impl TextLayout {
    pub fn size(&self) -> V2 {
        self.size
    }
}

fn layout_paragraph<'a>(font: &rusttype::Font<'a>, scale: Scale, width: i32, text: &str) -> (V2, Vec<PositionedGlyph<'a>>) {
    let mut result = Vec::new();
    let v_metrics = font.v_metrics(scale);
    let advance_height = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;

    let mut caret = point(0.0, v_metrics.ascent);
    let mut max_width = 0.;
    let mut last_glyph_id = None;

    for c in text.chars() {
        if c.is_control() {
            match c {
                '\r' => caret = point(0.0, caret.y + advance_height),
                '\n' => {}
                _ => {}
            }
            continue;
        }

        let base_glyph = font.glyph(c);
        if let Some(id) = last_glyph_id.take() {
            caret.x += font.pair_kerning(scale, id, base_glyph.id());
        }
        max_width = f32::max(max_width, caret.x);

        last_glyph_id = Some(base_glyph.id());
        let mut glyph = base_glyph.scaled(scale).positioned(caret);
        if let Some(bb) = glyph.pixel_bounding_box() {
            if bb.max.x > width as i32 {
                caret = point(0.0, caret.y + advance_height);
                glyph.set_position(caret);
                last_glyph_id = None;
            }
        }
        caret.x += glyph.unpositioned().h_metrics().advance_width;
        max_width = f32::max(max_width, caret.x);

        result.push(glyph);
    }
    (V2::new(max_width, caret.y), result)
}