use crate::gust::tower::Tower;
use crate::assets::Fonts;
use crate::{UpdateState, V2};
use crate::graphics::{Graphics, Color};
use crate::utils::{Rectangle, Tween};
use crate::input::Actions;
use crate::ui::{WidgetHelper, DockContainer, Label, HorizontalAlignment, VerticalAlignment};

mod manager;
pub use manager::Manager;

pub enum ModifierType {
    Flat(f32),
    Percent(f32),
}

pub struct Modifier {
    modifier_type: ModifierType,
    attribute: &'static str,
}
impl Modifier {
    pub fn new(modifier_type: ModifierType, attribute: &'static str) -> Modifier {
        Modifier { modifier_type, attribute }
    }
}

pub struct Card {
    modifiers: Vec<Modifier>,
    custom_offset: V2,
    drag_offset: V2,
    dragging: bool
}
impl Card {
    pub fn new(modifiers: Vec<Modifier>) -> Card {
        Card {
            modifiers,
            custom_offset: V2::new(0., 0.),
            drag_offset: V2::new(0., 0.),
            dragging: false
        }
    }

    pub fn draw(deck: &mut Vec<Card>, root: &mut crate::ui::Widget) {
        if let Some(card) = deck.pop() {
            root.with_child(card)
                .with_h_alignment(HorizontalAlignment::Custom)
                .with_v_alignment(VerticalAlignment::Custom)
                .with_child(DockContainer::vertical(1., 1., Some(Color::GRAY)))
                    .add_child(Label::new("Card!".to_string(), Fonts::Regular, 16., Color::WHITE));
        }
    }

    pub fn apply_tower(&self, tower: &mut Tower) {
        for m in &self.modifiers {
            let current = Card::get_tower_attribute(tower, m.attribute);
            let set = Card::get_set_tower_attribute(tower, m.attribute);
            match m.modifier_type {
                ModifierType::Flat(a) => set(tower, current + a),
                ModifierType::Percent(p) => set(tower, current + (current * p)),
            }
        }
    }

    fn get_tower_attribute(tower: &mut Tower, attribute: &'static str) -> f32 {
        match attribute {
            "attack_speed" => tower.attack_speed(),
            "range" => tower.range(),
            "damage" => tower.damage(),
            _ => panic!("Unknown attribute modifier for tower")
        }
    }
    fn get_set_tower_attribute(tower: &mut Tower, attribute: &'static str) -> impl Fn(&mut Tower, f32) {
        match attribute {
            "attack_speed" => Tower::set_attack_speed,
            "range" => Tower::set_range,
            "damage" => Tower::set_damage,
            _ => panic!("Unknown attribute modifier for tower")
        }
    }
}
impl crate::ui::UiElement for Card {
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }

    fn custom_offset(&self) -> V2 {
        self.custom_offset
    }

    fn layout(&mut self, rect: &Rectangle, helper: &mut WidgetHelper) -> V2 {
        let size = V2::new(100., 200.);
        let rect = Rectangle::new(rect.top_left(), size);
        helper.layout_children(&rect);

        size
    }

    fn render(&self, _graphics: &mut Graphics, _rect: &Rectangle) { }

    fn update(&mut self, state: &mut UpdateState, helper: &mut WidgetHelper, rect: &Rectangle) {
        let mouse_pos = state.mouse_pos();
        if self.dragging {
            self.custom_offset = mouse_pos - self.drag_offset;

            if state.action_released(Actions::Select) {
                self.dragging = false;
                
                let targets = state.quad_tree.within_distance::<crate::gust::tower::Tower>(mouse_pos, 50., state.entities);
                if targets.len() == 1 {
                    let tower = crate::utils::entity_as_mut::<crate::gust::tower::Tower>(state.entities.get_mut(&targets[0]).unwrap()).unwrap();
                    self.apply_tower(tower);
                    helper.destroy();
                }
            }
        } else {
            let x = Tween::lerp(self.custom_offset.x, 0., state.delta_time * 5.);
            let y = Tween::lerp(self.custom_offset.y, 0., state.delta_time * 5.);
            self.custom_offset = V2::new(x, y);

            if rect.contains(mouse_pos) {
                if state.action_pressed(Actions::Select) {
                    state.consume_action(Actions::Select);
                    self.drag_offset = state.mouse_pos() - self.custom_offset;
                    self.dragging = true;
                }
            }
        }
    }
}