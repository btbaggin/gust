#![allow(dead_code)]

use crate::V2;
use cgmath::{Matrix2, InnerSpace, SquareMatrix};
use std::f32::EPSILON;

pub trait Shape {
    fn set_orient(&mut self, radians: f32);
}

pub enum CollisionShape {
    Circle(Circle),
    Polygon(Polygon),
}
impl CollisionShape {
    pub fn set_orient(&mut self, rotation: f32) {
        match self {
            CollisionShape::Circle(c) => c.set_orient(rotation),
            CollisionShape::Polygon(p) => p.set_orient(rotation),
        }
    }
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
    pub(super) u: Matrix2<f32>,
    vertices: Vec<V2>,
    normals: Vec<V2>,
}
impl Polygon {
    pub fn new(vertices: Vec<V2>) -> Polygon {
        let (vertices, normals) = Polygon::get_vertices(vertices);
        Polygon {
            u: Matrix2::identity(),
            vertices,
            normals,
        }
    }
    pub fn rectangle(width: f32, height: f32, offset: V2) -> Polygon {
        let vertices = vec!(offset, V2::new(offset.x + width, offset.y),
                            V2::new(offset.x + width, offset.y + height), V2::new(offset.x, offset.y + height));
        let normals = vec!(V2::new(0., -1.), V2::new(1., 0.), V2::new(0., 1.), V2::new(-1., 0.));
        Polygon {
            u: Matrix2::identity(),
            vertices,
            normals,
        }
    }

    pub fn vertices(&self) -> &Vec<V2> { &self.vertices }
    pub fn normals(&self) -> &Vec<V2> { &self.normals }

    fn get_vertices(in_vertices: Vec<V2>) -> (Vec<V2>, Vec<V2>) {
        let mut right_most = 0;
        let mut highest_x_coord = in_vertices[0].x;
        for i in 1..in_vertices.len() {
            let x = in_vertices[i].x;
            if x > highest_x_coord {
                highest_x_coord = x;
                right_most = i;
            }
            else if x == highest_x_coord && in_vertices[i].y < in_vertices[right_most].y {
                right_most = i;
            }
        }

        let mut hull = vec!();
        let mut out_count = 0;
        let mut index_hull = right_most;

        loop {
            hull.push(index_hull);

            // Search for next index that wraps around the hull
            // by computing cross products to find the most counter-clockwise
            // vertex in the set, given the previos hull index
            let mut next_hull_index = 0;
            for i in 0..in_vertices.len() {
                // Skip if same coordinate as we need three unique
                // points in the set to perform a cross product
                if next_hull_index == index_hull {
                    next_hull_index = i;
                    continue;
                }

                // Cross every set of three unique vertices
                // Record each counter clockwise third vertex and add
                // to the output hull
                // See : http://www.oocities.org/pcgpe/math2d.html
                let e1 = in_vertices[next_hull_index] - in_vertices[hull[out_count]];
                let e2 = in_vertices[i] - in_vertices[hull[out_count]];
                let c = crate::utils::cross_v2(e1, e2);
                if c < 0. { next_hull_index = i; }

                // Cross product is zero then e vectors are on same line
                // therefor want to record vertex farthest along that line
                if c == 0. && e2.magnitude2() > e1.magnitude2() { next_hull_index = i; }
            }

            out_count += 1;
            index_hull = next_hull_index;

            // Conclude algorithm upon wrap-around
            if next_hull_index == right_most {
                break;
            }
        }

        let mut out_vertices = Vec::with_capacity(out_count);
        let mut normals = Vec::with_capacity(out_count);

        // Copy vertices into shape's vertices
        for i in 0..out_count {
            out_vertices[i] = in_vertices[hull[i]];
        }

        // Compute face normals
        for i1 in 0..out_count {
            let i2 = if i1 + 1 < out_count { i1 + 1 } else { 0 };
            let face = out_vertices[i2] - out_vertices[i1];

            // Ensure no zero-length edges, because that's bad
            assert!(face.magnitude2() > EPSILON * EPSILON);

            // Calculate normal with 2D cross product between vector and scalar
            normals[i1] = V2::new(face.y, -face.x);
            normals[i1] = normals[i1].normalize();
        }

        (out_vertices, normals)
    }

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
        let c = radians.cos();
        let s = radians.sin();

        self.u.x.x = c;
        self.u.x.y = -s;
        self.u.y.x = s;
        self.u.y.y = c;
    }
}