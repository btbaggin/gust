use crate::V2;
use crate::entity::Entity;
use crate::physics::{Polygon, Circle, Manifold};
use cgmath::{InnerSpace, Matrix, MetricSpace};

pub fn circle_to_circle(manifold: &mut Manifold, entity_a: &Entity, entity_b: &Entity, a: &Circle, b: &Circle) {
	// Calculate translational vector, which is normal
	let normal = entity_b.position - entity_a.position;

	let dist_sqr = normal.magnitude2();
	let radius = a.radius() + b.radius();

	// Not in contact
	if dist_sqr >= radius * radius {
		manifold.contact_count = 0;
		return;
	}

	let distance = f32::sqrt(dist_sqr);
	manifold.contact_count = 1;

	if distance == 0. {
		manifold.penetration = a.radius();
		manifold.normal = V2::new(1., 0.);
		manifold.contacts[0] = entity_a.position;
	} else {
		manifold.penetration = radius - distance;
		manifold.normal = normal / distance; // Faster than using Normalized since we already performed sqrt
		manifold.contacts[0] = manifold.normal * a.radius() + entity_a.position;
	}
}

pub fn circle_to_polygon(manifold: &mut Manifold, entity_a: &Entity, entity_b: &Entity, a: &Circle, b: &Polygon) {
	manifold.contact_count = 0;

	// Transform circle center to Polygon model space
	let center = entity_a.position;
	let center = b.u.transpose() * (center - entity_b.position);

    let vertices = b.vertices();
    let normals = b.normals();
	// Find edge with minimum penetration
	// Exact concept as using support points in Polygon vs Polygon
	let mut separation = f32::MIN;
	let mut face_normal = 0;
	for i in 0..vertices.len() {
		let s = normals[i].dot(center - vertices[i]);
		if s > a.radius() { return; }

		if s > separation {
			separation = s;
			face_normal = i;
		}
	}

	// Grab face's vertices
	let i2 = if face_normal + 1 < vertices.len() { face_normal + 1 } else { 0 };
	let mut v1 = vertices[face_normal];
	let mut v2 = vertices[i2];

	// Check to see if center is within polygon
	if separation < f32::EPSILON {
		manifold.contact_count = 1;
		manifold.normal = (b.u * normals[face_normal]) * -1.;
		manifold.contacts[0] = manifold.normal * a.radius() + entity_a.position;
		manifold.penetration = a.radius();
		return;
	}

	// Determine which voronoi region of the edge center of circle lies within
	let dot1 = (center - v1).dot(v2 - v1);
	let dot2 = (center - v2).dot(v1 - v2);
 	manifold.penetration = a.radius() - separation;

     if dot1 <= 0. {
        // Closest to v1
		if center.distance2(v1) > a.radius() * a.radius() { return; }

		manifold.contact_count = 1;
		let n = v1 - center;
		let n = b.u * n;
		manifold.normal = n.normalize();
		v1 = b.u * v1 + entity_b.position;
		manifold.contacts[0] = v1;
	} else if dot2 <= 0. { 
        // Closest to v2
		if center.distance2(v2) > a.radius() * a.radius() { return; }

		manifold.contact_count = 1;
		let n = v2 - center;
		let n = b.u * n;
		manifold.normal = n.normalize();
		v2 = b.u * v2 + entity_b.position;
		manifold.contacts[0] = v2;
	} else  {
        // Closest to face
		let n = normals[face_normal];
		if (center - v1).dot(n) > a.radius() { return; }
        
		let n = b.u * n;
		manifold.normal = n * -1.;
		manifold.contacts[0] = manifold.normal * a.radius() + entity_a.position;
		manifold.contact_count = 1;
	}
}

fn find_axis_least_penetration(position_a: V2, position_b: V2, a: &Polygon, b: &Polygon) -> (f32, usize) {
	let mut best_distance = f32::MIN;
	let mut best_index = 0;

	for i in 0..a.vertices().len() {
		// Retrieve a face normal from A
		let n = a.normals()[i];
		let nw = a.u * n;

		// Transform face normal into B's model space
		let bu_t = b.u.transpose();
		let n = bu_t * nw;

		// Retrieve support point from B along -n
		let s = b.get_support(n * -1.);

		// Retrieve vertex on face from A, transform into
		// B's model space
		let v = a.vertices()[i];
		let v = a.u * v + position_a;
		let v = v - position_b;
		let v = bu_t * v;

		// Compute penetration distance (in B's model space)
		let d = n.dot(s - v);

		// Store greatest distance
		if d > best_distance {
			best_distance = d;
			best_index = i;
		}
	}

	(best_distance, best_index)
}

fn find_incident_face(ref_poly: &Polygon, inc_poly: &Polygon, inc_poly_position: V2, index: usize) -> [V2; 2] {
	let reference_normal = ref_poly.normals()[index];

	// Calculate normal in incident's frame of reference
	let reference_normal = ref_poly.u * reference_normal; // To world space
	let reference_normal =  inc_poly.u * reference_normal; // To incident's model space

	// Find most anti-normal face on incident polygon
	let mut incident_face = 0;
	let mut min_dot = f32::MAX;
	for i in 0..inc_poly.vertices().len() {
		let dot = reference_normal.dot(inc_poly.normals()[i]);
		if dot < min_dot {
			min_dot = dot;
			incident_face = i;
		}
	}

    let mut ret = [V2::new(0., 0.); 2];
	// Assign face vertices for incidentFace
	ret[0] = inc_poly.u * inc_poly.vertices()[incident_face] + inc_poly_position;
	let incident_face = if incident_face + 1 >= inc_poly.vertices().len() { 0 } else { incident_face + 1 };
	ret[1] = inc_poly.u * inc_poly.vertices()[incident_face] + inc_poly_position;

    ret
}

fn clip(normal: V2, c: f32, face: [V2; 2]) -> (usize, [V2; 2]) {
	let mut sp = 0;
	let mut out = [V2::new(0., 0.); 2];
    out[0] = face[0];
    out[1] = face[1];

	// Retrieve distances from each endpoint to the line
	// d = ax + by - c
	let d1 = normal.dot(face[0]) - c;
	let d2 = normal.dot(face[1]) - c;

	// If negative (behind plane) clip
	if d1 <= 0. { 
        out[sp] = face[0];
        sp += 1;
    }
	if d2 <= 0. { 
        out[sp] = face[1];
        sp += 1;
    }

	// If the points are on different sides of the plane
	if d1 * d2 < 0. { // less than to ignore -0.0f
		// Push interesection point
		let alpha = d1 / (d1 - d2);
		out[sp] = face[0] + alpha * (face[1] - face[0]);
		sp += 1;
	}

	assert!(sp != 3);
	(sp, out)
}

fn bias_greater_than(a: f32, b: f32) -> bool {
	a >= b * 0.95 + a * 0.01
}

pub fn polygon_to_polygon(manifold: &mut Manifold, entity_a: &Entity, entity_b: &Entity, a: &Polygon, b: &Polygon) {
	manifold.contact_count = 0;

	// Check for a separating axis with A's face planes
	let (penetration_a, face_a) = find_axis_least_penetration(entity_a.position, entity_b.position, a, b);
	if penetration_a >= 0. { return; }

	// Check for a separating axis with B's face planes
	let (penetration_b, face_b) = find_axis_least_penetration(entity_b.position, entity_a.position, b, a);
	if penetration_b >= 0. { return; }


	// Determine which shape contains reference face
	let (ref_poly, inc_poly, ref_pos, inc_pos, reference_index, flip) = 
    if bias_greater_than(penetration_a, penetration_b) {
		(a, b, entity_a.position, entity_b.position, face_a, false)
	} else {
        (b, a, entity_b.position, entity_a.position, face_b, true)
	};

	// World space incident face
	let incident_face = find_incident_face(ref_poly, inc_poly, inc_pos, reference_index);

	//        y
	//        ^  ->n       ^
	//      +---c ------posPlane--
	//  x < | i |\
    //      +---+ c-----negPlane--
	//             \       v
	//              r
	//
	//  r : reference face
	//  i : incident poly
	//  c : clipped point
	//  n : incident normal

	// Setup reference face vertices
	let v1 = ref_poly.vertices()[reference_index];
	let reference_index = if reference_index + 1 == ref_poly.vertices().len() { 0 } else { reference_index + 1 };
	let v2 = ref_poly.vertices()[reference_index];

	// Transform vertices to world space
	let v1 = ref_poly.u * v1 + ref_pos;
	let v2 = ref_poly.u * v2 + ref_pos;

	// Calculate reference face side normal in world space
	let side_plane_normal = v2 - v1;
	let side_plane_normal = side_plane_normal.normalize();

	// Orthogonalize
	let ref_face_normal = V2::new(side_plane_normal.y, -side_plane_normal.x);

	// ax + by = c
	// c is distance from origin
	let ref_c = ref_face_normal.dot(v1);
	let neg_side = -side_plane_normal.dot(v1);
	let pos_side = side_plane_normal.dot(v2);

	// Clip incident face to reference face side planes
    // Due to floating point error, possible to not have required points
    let (sides, incident_face) = clip(side_plane_normal * -1., neg_side, incident_face);
    if sides < 2 { return; }
	let (sides, incident_face) = clip(side_plane_normal, pos_side, incident_face);
    if sides < 2 { return; }

	// Flip
	manifold.normal = if flip { ref_face_normal * -1. } else { ref_face_normal };

	// Keep points behind reference face
	let mut cp = 0; // clipped points behind reference face
	let separation = ref_face_normal.dot(incident_face[0]) - ref_c;
	if separation <= 0. {
		manifold.contacts[cp] = incident_face[0];
		manifold.penetration = -separation;
		cp += 1;
	} else {
        manifold.penetration = 0.;
    }

	let separation = ref_face_normal.dot(incident_face[1]) - ref_c;
	if separation <= 0. {
		manifold.contacts[cp] = incident_face[1];

		manifold.penetration += -separation;
		cp += 1;

		// Average penetration
		manifold.penetration /= cp as f32;
	}

	manifold.contact_count = cp;
}