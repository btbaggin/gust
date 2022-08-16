#![allow(dead_code)]

use crate::V2;
use cgmath::Matrix2;
use cgmath::InnerSpace;

pub trait Shape {
    fn set_orient(&mut self, radians: f32);
}

pub enum CollisionShape {
    Circle(Circle),
    Polygon(Polygon),
}

pub struct Circle {
    radius: f32,
}
impl Circle {
    pub fn new(radius: f32) -> Circle {
        Circle { radius }
    }
    pub fn radius(&self) -> f32 { self.radius }
    pub fn set_radius(&mut self, radius: f32) { self.radius = radius }
}
impl Shape for Circle {
    fn set_orient(&mut self, radians: f32) { }
}
pub struct Polygon {
    //TODO matrix,
    pub(super) u: Matrix2<f32>,
    vertices: Vec<V2>,
    normals: Vec<V2>,
}
impl Polygon {
    pub fn vertices(&self) -> &Vec<V2> { &self.vertices }
    pub fn normals(&self) -> &Vec<V2> { &self.normals }

    pub(super) fn get_support(&self, dir: V2) -> V2 {
        let mut best_projection = f32::MIN;
        let mut best_vertex = V2::new(0., 0.);

        for v in &self.vertices {
            let projection = v.dot(dir);
            if projection > best_projection {
                best_vertex = *v;
                best_projection = projection;
            }
        }

        best_vertex
    }

    pub(super) fn calulate_mass_inertia(&self, density: f32) -> (f32, f32) {
        let mut area = 0.;
        let mut i = 0.;
        const K_INV3: f32 = 1. / 3.;
    
        for i1 in 0..self.vertices.len() {
            let i2 = if i1 + 1 < self.vertices.len() { i1 + 1 } else { 0 };
            let p1 = self.vertices[i1];
            let p2 = self.vertices[i2];
    
            let d = crate::utils::cross_v2(p1, p2);
            let triangle_area = 0.5 * d;
    
            area += triangle_area;
    
            let intx2 = p1.x * p1.x + p2.x * p1.x + p2.x * p2.x;
            let inty2 = p1.y * p1.y + p2.y * p1.y + p2.y * p2.y;
            i += (0.25 * K_INV3 * d) * (intx2 + inty2);
        }
    
        (density * area, density * i)
    }
}
impl Shape for Polygon {
    fn set_orient(&mut self, radians: f32) {
        //TODO
    }
}