use std::time::Instant;
use std::rc::Rc;
use std::cell::RefCell;
use glutin::dpi::PhysicalSize;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::{Fullscreen, WindowBuilder};
use crate::V2;
use crate::entity::{SceneBehavior};
use crate::{input::Input, job_system::ThreadSafeJobQueue};
use crate::messages::MessageBus;
use crate::graphics::Graphics;
use crate::physics::QuadTree;
use crate::game_window::{WindowHandler, create_window};
use crate::utils::Rectangle;

pub struct GlobalState {
    pub screen_size: V2,
}
impl GlobalState {
    fn new(screen_size: V2) -> GlobalState {
        GlobalState { screen_size }
    }
}

static mut GLOBAL_STATE_VAR: Option<GlobalState> = None;
pub fn global_state<'a>() -> &'a GlobalState {
    unsafe { GLOBAL_STATE_VAR.as_ref().unwrap() }
}

pub fn start_game_loop<H>(title: &'static str, size: Option<(f32, f32)>, target_frames: u32,
                          mut input: Input, queue: ThreadSafeJobQueue, scene: Box<dyn SceneBehavior>, 
                          mut handler: H) -> ! 
    where H: WindowHandler + 'static {
    let el = EventLoop::new();

    //Build windows
    let monitor = el.primary_monitor();
    let builder = WindowBuilder::new()
        .with_title(title)  
        .with_visible(true);
    let builder = match size {
        Some(s) => builder.with_inner_size(PhysicalSize::new(s.0, s.1)),
        None => builder.with_fullscreen(Some(Fullscreen::Borderless(monitor))),
    };

    let (context, mut window, size) = create_window(&el, builder);

    let expected_seconds_per_frame = 1. / target_frames as f32;

    //Initialize game state
    unsafe {
        use std::borrow::BorrowMut;
        *GLOBAL_STATE_VAR.borrow_mut() = Some(GlobalState::new(size));
    }


    let mut last_time = Instant::now();
    let mut mouse_position = crate::V2::new(0., 0.);

    let message_bus = Rc::new(RefCell::new(MessageBus::new()));
    let entities = crate::entity::entity_manager();

    let bounds = Rectangle::new(V2::new(0., 0.), size);
    let mut quad_tree = QuadTree::new(bounds);

 
    //Start scene
    let mut scene = crate::entity::Scene::new(scene);
    scene.load(queue.clone(), message_bus.clone(), entities);
    
    //Run game
    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        let global_state = unsafe { GLOBAL_STATE_VAR.as_mut().unwrap() };
        
        match event {
            Event::LoopDestroyed => *control_flow = ControlFlow::Exit,

            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    handler.on_stop();
                },

                WindowEvent::Resized(physical_size) => {
                    global_state.screen_size = V2::new(physical_size.width as f32, physical_size.height as f32);

                    let bounds = Rectangle::new(V2::new(0., 0.), global_state.screen_size);
                    quad_tree = QuadTree::new(bounds);

                    context.resize(physical_size);
                    window.set_viewport_size_pixels(speedy2d::dimen::Vector2::new(physical_size.width, physical_size.height));
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

                let mut state = crate::UpdateState::new(
                    delta_time,
                    &input,
                    message_bus.clone(),
                    queue.clone(),
                    entities,
                    &quad_tree
                );
                if !handler.on_update(&mut state, &mut scene) {
                    *control_flow = ControlFlow::Exit;
                    handler.on_stop();
                }

                let mut messages = message_bus.borrow_mut();
                unsafe { crate::physics::step_physics(expected_seconds_per_frame, &mut messages); }

                window.draw_frame(|graphics| {
                    let mut graphics = Graphics::new(graphics, queue.clone());
                    handler.on_render(&mut graphics, &scene, entities);
                });
                context.swap_buffers().unwrap();
                
                // TODO skip on frame end if game is running slow
                handler.on_frame_end();
                
                entities.dispose_entities(&mut messages);
                quad_tree.update_positions(entities);

                sleep_until_frame_end(now, expected_seconds_per_frame);
            },
            _ => {}
        }
    });
}

fn sleep_until_frame_end(start: Instant, expected_seconds_per_frame: f32) {
    let now = Instant::now();
	let update_seconds = (now - start).as_secs_f32();

	if  update_seconds < expected_seconds_per_frame {
        let sleep_time = expected_seconds_per_frame - update_seconds;
        std::thread::sleep(std::time::Duration::from_secs_f32(sleep_time));
	}
}