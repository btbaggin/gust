use crate::V2;
use crate::entity::{EntityHandle, EntityManager};
use crate::utils::{Rectangle, init_optional_array_to_blank};
use cgmath::MetricSpace;

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

    pub fn within_distance<T: 'static>(&self, position: V2, distance: f32, entities: &EntityManager) -> Vec<EntityHandle> {
        let rect = Rectangle::new(V2::new(position.x - distance, position.y - distance), V2::new(distance * 2., distance * 2.));
        let close = self.find(rect);
        let mut results = vec!();
        for e in close {
            let entity = entities.get(e).unwrap();
            if entity.position.distance2(position) < distance * distance &&
               entity.as_any().downcast_ref::<T>().is_some() {
                results.push(*e);
            }
        }
        results
    }

    pub fn at_point<T: 'static>(&self, point: V2, entities: &EntityManager) -> Vec<EntityHandle> {
        let rect = Rectangle::new(point, point);
        let close = self.find(rect);
        let mut results = vec!();
        for e in close {
            let entity = entities.get(e).unwrap();
            if entity.bounds().contains(point) &&
               entity.as_any().downcast_ref::<T>().is_some() {
                results.push(*e)
            }
        }
        results
    }

    pub fn update_positions(&mut self, entities: &mut EntityManager) {
        //QuadTree will be one frame behind but that's fine because it's only used for course detection
        self.clear();
        for h in entities.iter_handles() {
            let entity = entities.get(&h).unwrap();
            let bounds = Rectangle::new(entity.position, entity.scale);
            self.insert(h, bounds)
        }
    }

    fn find(&self, bounds: Rectangle) -> &Vec<EntityHandle> {
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

    fn clear(&mut self) {
        self.root.entities.clear();
        for n in &mut self.nodes.iter_mut().flatten() {
            n.entities.clear();
        }
    }

    fn insert(&mut self, entity: EntityHandle, bounds: Rectangle) {
        //TODO quad_tree broken somehow
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

        let top_left = bounds.top_left();
        let bottom_right = bounds.bottom_right();
        let mid_point = half_size + self.bounds.top_left();

        fn return_index_if_rect_fits(self_bottom_right: V2, bottom_right: V2, index: usize) -> Option<usize> {
            if bottom_right.x < self_bottom_right.x && bottom_right.y < self_bottom_right.y { 
                Some(index)
            } else {
                None
            }
        }

        if top_left.y < mid_point.y {
            if top_left.x < mid_point.x { 
                return_index_if_rect_fits(mid_point, bottom_right, 0)
            } else {
                return_index_if_rect_fits(V2::new(mid_point.x + half_size.x, mid_point.y), bottom_right, 1)
            }
        } else {
            if top_left.x < mid_point.x {
                return_index_if_rect_fits(V2::new(mid_point.x, mid_point.y + half_size.y), bottom_right, 2)
            } else {
                return_index_if_rect_fits(mid_point + half_size, bottom_right, 3)
            }
        }
    }

    fn split(&self, index: usize) -> QuadTreeNode {
        let top_left = self.bounds.top_left();
        let half_size = self.bounds.size() / 2.;
        let bounds = match index {
            0 => Rectangle::new(top_left, half_size),
            1 => Rectangle::new(V2::new(top_left.x + half_size.x, top_left.y), half_size),
            2 => Rectangle::new(V2::new(top_left.x, top_left.y + half_size.y), half_size),
            3 => Rectangle::new(top_left + half_size, half_size),
            _ => panic!("Invalid index"),
        };
        
        QuadTreeNode::new(bounds)
    }
}