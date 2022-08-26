use std::time::Instant;
use std::rc::Rc;
use std::cell::RefCell;
use speedy2d::*;
use glutin::dpi::PhysicalSize;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::{Fullscreen, WindowBuilder};
use glutin::{PossiblyCurrent, ContextWrapper};
use crate::entity::{SceneBehavior, Scene};
use crate::{input::Input, job_system::ThreadSafeJobQueue};
use crate::messages::{MessageBus, SharedMessageBus};
use crate::graphics::Graphics;

pub struct UpdateState<'a> {
    pub delta_time: f32,
    pub input: &'a Input,
    pub message_bus: SharedMessageBus,
    pub queue: ThreadSafeJobQueue,
}

pub trait WindowHandler {
    // fn on_start(&mut self) { }
    fn on_update(&mut self, state: &mut UpdateState, scene: &mut Scene) -> bool;
    fn on_render(&mut self, graphics: &mut Graphics, scene_manager: &Scene, delta_time: f32, size: PhysicalSize<u32>);
    fn on_frame_end(&mut self) { }
    fn on_resize(&mut self, _: u32, _: u32) { }
    fn on_focus(&mut self, _: bool) { }
    fn on_stop(&mut self) { }
}

struct GameWindow {
    renderer: GLRenderer,
    size: PhysicalSize<u32>,
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

fn create_window(event_loop: &EventLoop<()>, 
                 builder: WindowBuilder) -> (ContextWrapper<PossiblyCurrent, glutin::window::Window>, GameWindow) {

    use crate::logger::PanicLogEntry;
    let context = create_best_context(&builder, event_loop).log_and_panic();
    let context = unsafe { context.make_current().unwrap() };

    let size = context.window().inner_size();
    let renderer = unsafe {
        GLRenderer::new_for_gl_context((size.width, size.height), |fn_name| {
            context.get_proc_address(fn_name) as *const _
        })
    }.unwrap();

    (context, GameWindow { renderer, size })
}

pub(crate) fn create_game_window<H>(title: &'static str, fullscreen: bool, mut input: Input, 
                                    queue: ThreadSafeJobQueue, scene: Box<dyn SceneBehavior>, 
                                    mut handler: H) -> ! 
    where H: WindowHandler + 'static {
    let el = EventLoop::new();

    let monitor = el.primary_monitor();
    let fullscreen = if fullscreen { Some(Fullscreen::Borderless(monitor)) } else { None };
    let builder = WindowBuilder::new()
        .with_title(title)  
        .with_fullscreen(fullscreen)
        .with_visible(true);
    let (context, mut window) = create_window(&el, builder);

    let mut last_time = Instant::now();
    let mut mouse_position = crate::V2::new(0., 0.);

    let message_bus = Rc::new(RefCell::new(MessageBus::new()));

    let mut scene = crate::entity::Scene::new(scene);
    scene.load(queue.clone(), message_bus.clone());
    
    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        
        match event {
            Event::LoopDestroyed => *control_flow = ControlFlow::Exit,

            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    handler.on_stop();
                },

                WindowEvent::Resized(physical_size) => {
                    window.size = PhysicalSize::new(physical_size.width, physical_size.height);

                    context.resize(physical_size);
                    window.renderer.set_viewport_size_pixels(speedy2d::dimen::Vector2::new(physical_size.width, physical_size.height));
                    handler.on_resize(physical_size.width, physical_size.height);
                },
                WindowEvent::Focused(focused) => {
                    handler.on_focus(focused)
                },
                WindowEvent::ReceivedCharacter(_) => { }
                WindowEvent::CursorMoved { position, .. } => { mouse_position = crate::V2::new(position.x as f32, position.y as f32); }
                _ => {}
            },

            Event::MainEventsCleared => {
                let now = Instant::now();
                let delta_time = (now - last_time).as_millis() as f32 / 1000.;
                last_time = now;

                crate::input::gather(&mut input, mouse_position);

                let mut state = UpdateState {
                    delta_time,
                    input: &input,
                    message_bus: message_bus.clone(),
                    queue: queue.clone(),
                };
                if !handler.on_update(&mut state, &mut scene) {
                    *control_flow = ControlFlow::Exit;
                    handler.on_stop();
                }

                unsafe { crate::physics::step_physics(0.2); } //TODO don't hardcode time, need to calculate from target fps

                let size = PhysicalSize::new(window.size.width, window.size.height);
                window.renderer.draw_frame(|graphics| {
                    let mut graphics = Graphics { graphics, queue: queue.clone() };
                    handler.on_render(&mut graphics, &scene, delta_time, size);
                });
                context.swap_buffers().unwrap();

                // TODO skip on frame end if game is running slow
                handler.on_frame_end();
                let entity_manager = crate::entity::entity_manager();
                let mut m = message_bus.borrow_mut();
                entity_manager.dispose_entities(&mut m);
            },
            _ => {}
        }
    });
}