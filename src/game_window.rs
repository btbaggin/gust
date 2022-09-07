use glutin::{PossiblyCurrent, ContextWrapper};
use speedy2d::*;
use crate::entity::{Scene, EntityManager};
use crate::graphics::Graphics;
use glutin::window::WindowBuilder;
use glutin::event_loop::EventLoop;

pub trait WindowHandler {
    // fn on_start(&mut self) { }
    fn on_update(&mut self, state: &mut crate::UpdateState, scene: &mut Scene) -> bool;
    fn on_render(&mut self, graphics: &mut Graphics, scene_manager: &Scene, entities: &EntityManager);
    fn on_frame_end(&mut self) { }
    fn on_resize(&mut self, _: u32, _: u32) { }
    fn on_focus(&mut self, _: bool) { }
    fn on_stop(&mut self) { }
}

fn create_best_context(window_builder: &WindowBuilder, event_loop: &EventLoop<()>) -> Option<glutin::WindowedContext<glutin::NotCurrent>> {
    for vsync in &[true, false] {
        for multisampling in &[8, 4, 2, 1, 0] {

            let mut windowed_context = glutin::ContextBuilder::new()
                .with_vsync(*vsync)
                .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (2, 0)));

            if *multisampling > 1 {
                windowed_context = windowed_context.with_multisampling(*multisampling);
            }

            let result = windowed_context.build_windowed(window_builder.clone(), event_loop);

            match result {
                Ok(context) => { return Some(context); }
                Err(err) => {
                    crate::logger::warn!("Failed to create context: {:?}", err);
                }
            }
        }
    }

    None
}

pub fn create_window(event_loop: &EventLoop<()>, 
                 builder: WindowBuilder) -> (ContextWrapper<PossiblyCurrent, glutin::window::Window>, GLRenderer, crate::V2) {

    use crate::logger::PanicLogEntry;
    let context = create_best_context(&builder, event_loop).log_and_panic();
    let context = unsafe { context.make_current().unwrap() };

    let size = context.window().inner_size();
    let renderer = unsafe {
        GLRenderer::new_for_gl_context((size.width, size.height), |fn_name| {
            context.get_proc_address(fn_name) as *const _
        })
    }.unwrap();

    (context, renderer, crate::V2::new(size.width as f32, size.height as f32))
}
