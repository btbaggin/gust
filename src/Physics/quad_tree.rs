use crate::entity::EntityHandle;
use crate::V2;
use speedy2d::{shape::Rectangle, dimen::Vector2};
use crate::utils::init_optional_array_to_blank;

const MAX_LEVELS: usize = 8;

pub struct QuadTree {
    root: QuadTreeNode,
    nodes: [Option<QuadTreeNode>; 4 * MAX_LEVELS],
}
struct QuadTreeNode {
    bounds: Rectangle,
    entities: Vec<EntityHandle>,
}
impl QuadTree {
    pub fn new(bounds: Rectangle) -> QuadTree {
        QuadTree {
            root: QuadTreeNode::new(bounds),
            nodes: init_optional_array_to_blank(),  //TODO this whole array could probably be initialized
        }
    }

    pub fn clear(&mut self) {
        self.root.entities.clear();
        for n in &mut self.nodes {
            if let Some(n) = n {
                n.entities.clear();
            }
        }
    }

    pub fn find(&self, bounds: Rectangle) -> &Vec<EntityHandle> {
        let mut node = &self.root;
        for i in 0..MAX_LEVELS {
            let relative_index = match node.get_index(&bounds) {
                Some(i) => i,
                None => return &node.entities,
            };

            let index = Self::calc_index(relative_index, i);
            node = match &self.nodes[index] {
                Some(n) => n,
                None => return &node.entities
            };
        }
        &node.entities
    }

    pub fn insert(&mut self, entity: EntityHandle, bounds: Rectangle) {
        let mut node = &mut self.root;
        for i in 0..MAX_LEVELS {
            node.entities.push(entity);

            let relative_index = match node.get_index(&bounds) {
                Some(i) => i,
                None => return,
            };

            let index = Self::calc_index(relative_index, i);
            if self.nodes[index].is_none() {
                self.nodes[index] = Some(node.split(relative_index));
            }
            node = unsafe { &mut *(self.nodes.get_unchecked_mut(index).as_mut().unwrap() as *mut _) };
        }
        node.entities.push(entity);
    }

    fn calc_index(index: usize, level: usize) -> usize { level * 4 + index }
}
impl QuadTreeNode {
    fn new(bounds: Rectangle) -> QuadTreeNode {
        QuadTreeNode {
            entities: vec!(),
            bounds,
        }
    }

    fn get_index(&self, bounds: &Rectangle) -> Option<usize> {
        let half_size = self.bounds.size() / 2.;
        if bounds.size().x > half_size.x || bounds.size().y > half_size.y {
            return None;
        }

        let top_left = *bounds.top_left();
        let bottom_right = *bounds.bottom_right();
        let self_bottom_right = *self.bounds.bottom_right();
        let mid_point = half_size + *self.bounds.top_left();

        fn return_index_if_rect_fits(self_bottom_right: Vector2<f32>, bottom_right: Vector2<f32>, index: usize) -> Option<usize> {
            if bottom_right.x < self_bottom_right.x && bottom_right.y < self_bottom_right.y { 
                Some(index)
            } else {
                None
            }
        }

        if top_left.y < mid_point.y {
            if top_left.x < mid_point.x { 
                return_index_if_rect_fits(self_bottom_right, bottom_right, 0)
            } else {
                return_index_if_rect_fits(self_bottom_right, bottom_right, 1)
            }
        } else {
            if top_left.x < mid_point.x {
                return_index_if_rect_fits(self_bottom_right, bottom_right, 2)
            } else {
                return_index_if_rect_fits(self_bottom_right, bottom_right, 3)
            }
        }
    }

    fn split(&self, index: usize) -> QuadTreeNode {
        let top_left = *self.bounds.top_left();
        let half_size = self.bounds.size() / 2.;
        let bounds = match index {
            0 => sized_speedy_rect(top_left, half_size),
            1 => sized_speedy_rect(Vector2::new(top_left.x + half_size.x, top_left.y), half_size),
            2 => sized_speedy_rect(Vector2::new(top_left.x, top_left.y + half_size.y), half_size),
            3 => sized_speedy_rect(top_left + half_size, half_size),
            _ => panic!("Invalid index"),
        };
        
        QuadTreeNode::new(bounds)
    }
}

fn sized_speedy_rect(pos: Vector2<f32>, size: Vector2<f32>) -> Rectangle {
    Rectangle::new(pos, pos + size)
}