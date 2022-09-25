use crate::gust::cards::Card;
use crate::utils::{Timer, Rectangle};
use crate::{UpdateState, Graphics, V2};
use crate::ui::WidgetHelper;

pub struct Manager {
    deck: Vec<Card>,
    draw_timer: Timer,
}
impl Manager {
    pub fn new(deck: Vec<Card>) -> Manager {
        Manager {
            deck,
            draw_timer: Timer::new(2.)
        }
    }
    
}
impl crate::ui::UiElement for Manager {
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }

    fn layout(&mut self, rect: &Rectangle, helper: &mut WidgetHelper) -> V2 {
        let width = rect.width() * 0.75;
        let height = rect.height() * 0.1;
        let size = V2::new(width, height);
        let rect = Rectangle::new(rect.top_left(), size);
        helper.layout_children(&rect);

        size
    }

    fn render(&self, _graphics: &mut Graphics, _rect: &Rectangle) { }

    fn update(&mut self, state: &mut UpdateState, helper: &mut WidgetHelper, _rect: &Rectangle) {
        if self.draw_timer.update(state.delta_time) {
            let root = crate::ui::root();
            Card::draw(&mut self.deck, root);
            //TODO need to crate UI elements after update finishes
        }
    }
}
// impl MessageHandler for Manager {
//     crate::handle_messages!();
//     fn process(&mut self, _message: &Messages) {}
// }