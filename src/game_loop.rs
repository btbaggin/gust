use std::time::Instant;
use std::rc::Rc;
use std::cell::RefCell;
use glium::glutin::dpi::PhysicalSize;
use glium::glutin::event::{Event, WindowEvent};
use glium::glutin::event_loop::ControlFlow;
use glium::glutin::window::{Fullscreen, WindowBuilder};
use crate::V2;
use crate::entity::{SceneBehavior};
use crate::input::Actions;
use crate::{input::Input, job_system::ThreadSafeJobQueue};
use crate::messages::MessageBus;
use crate::graphics::Graphics;
use crate::physics::QuadTree;
use crate::utils::Rectangle;
use crate::entity::{EntityManager, Scene};
use crate::ui::{Widget, Root};

pub struct GlobalState {
    pub screen_size: V2,
}
impl GlobalState {
    fn new(screen_size: V2) -> GlobalState {
        GlobalState { screen_size }
    }
}

pub trait WindowHandler {
    // fn on_start(&mut self) { }
    fn on_update(&mut self, state: &mut crate::UpdateState, root: &mut crate::ui::Widget, scene: &mut Scene) -> bool;
    fn on_render(&mut self, graphics: &mut Graphics, root: &crate::ui::Widget, scene: &Scene, entities: &EntityManager);
    fn on_frame_end(&mut self) { }
    fn on_resize(&mut self, _: u32, _: u32) { }
    fn on_focus(&mut self, _: bool) { }
    fn on_stop(&mut self) { }
}


static mut GLOBAL_STATE_VAR: Option<GlobalState> = None;
pub fn global_state<'a>() -> &'a GlobalState {
    unsafe { GLOBAL_STATE_VAR.as_ref().unwrap() }
}

pub fn start_game_loop<H>(title: &'static str,
                          size: Option<(f32, f32)>,
                          target_frames: u32,
                          mut input: Input<Actions>,
                          queue: ThreadSafeJobQueue,
                          scene: Box<dyn SceneBehavior>, 
                          mut handler: H) -> ! 
    where H: WindowHandler + 'static {
    let el = glium::glutin::event_loop::EventLoop::new();

    //Build windows
    let monitor = el.primary_monitor();
    let builder = WindowBuilder::new()
        .with_title(title)  
        .with_visible(true);
    let builder = match size {
        Some(s) => builder.with_inner_size(PhysicalSize::new(s.0, s.1)),
        None => builder.with_fullscreen(Some(Fullscreen::Borderless(monitor))),
    };

    let mut window = crate::graphics::create_window(&el, builder, queue.clone());
    let size = window.window_size();

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
    let mut root = Widget::new(Root {}, None);

    let bounds = Rectangle::new(V2::new(0., 0.), size);
    let mut quad_tree = QuadTree::new(bounds);
 
    //Start scene
    let mut scene = crate::entity::Scene::new(scene);
    scene.load(queue.clone(), message_bus.clone(), entities, &mut root);
    
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

                    window.resize(physical_size);
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
                    &quad_tree,
                );
                if !handler.on_update(&mut state, &mut root, &mut scene) {
                    *control_flow = ControlFlow::Exit;
                    handler.on_stop();
                }

                let mut messages = message_bus.borrow_mut();
                unsafe { crate::physics::step_physics(expected_seconds_per_frame, &mut messages); }

                handler.on_render(&mut window, &root, &scene, entities);
                window.draw_frame();
                
                // TODO skip on frame end if game is running slow
                handler.on_frame_end();
                
                entities.dispose_entities(&mut messages);
                quad_tree.update_positions(entities);
                root.clear();
                root.create_widgets();    

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