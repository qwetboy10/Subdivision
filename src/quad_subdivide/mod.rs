mod divider;
mod loader;
mod transform;

use crate::render::Vertex as RVertex;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use vecmath::*;

#[derive(Debug, Hash)]
pub struct Quad {
    vertices: [Vertex; 4],
}

impl Eq for Quad {}

impl PartialEq for Quad {
    fn eq(&self, other: &Self) -> bool {
        self.vertices == other.vertices
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    position: Vector3<f32>,
}

impl Eq for Vertex {}

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        (self.position[0] - other.position[0]).abs() < 1e-4
            && (self.position[1] - other.position[1]).abs() < 1e-4
            && (self.position[2] - other.position[2]).abs() < 1e-4
    }
}

impl Hash for Vertex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.position[0] as i32).hash(state);
        (self.position[1] as i32).hash(state);
        (self.position[2] as i32).hash(state);
    }
}

pub fn get_vertex_map(quads: &Vec<Quad>) -> HashMap<&Vertex, HashSet<&Quad>> {
    let mut vertices: HashMap<&Vertex, HashSet<&Quad>> = HashMap::new();

    for quad in quads {
        for v in &quad.vertices {
            if !vertices.contains_key(v) {
                vertices.insert(&v, HashSet::new());
            }
            vertices.get_mut(&v).unwrap().insert(quad);
        }
    }

    vertices
}

pub fn get_adjacent<'a>(
    a: &'a Vertex,
    b: &'a Vertex,
    me: &'a Quad,
    map: &HashMap<&'a Vertex, HashSet<&'a Quad>>,
) -> Option<&'a Quad> {
    map.get(&a)
        .unwrap()
        .intersection(map.get(&b).unwrap())
        .find(|&&x| x != me)
        .map(|x| *x)
}

pub fn get_adjacency(quads: &Vec<Quad>) -> HashMap<&Quad, [Option<&Quad>; 4]> {
    let mut map = HashMap::new();

    let mut vertices: HashMap<&Vertex, HashSet<&Quad>> = HashMap::new();

    for quad in quads {
        for v in &quad.vertices {
            if !vertices.contains_key(v) {
                vertices.insert(&v, HashSet::new());
            }
            vertices.get_mut(&v).unwrap().insert(quad);
        }
    }

    for quad in quads {
        let v1 = quad.vertices[0];
        let v2 = quad.vertices[1];
        let v3 = quad.vertices[2];
        let v4 = quad.vertices[3];
        let n1 = vertices
            .get(&v1)
            .unwrap()
            .intersection(vertices.get(&v2).unwrap())
            .find(|&&x| x != quad)
            .map(|x| *x);
        let n2 = vertices
            .get(&v2)
            .unwrap()
            .intersection(vertices.get(&v3).unwrap())
            .find(|&&x| x != quad)
            .map(|x| *x);
        let n3 = vertices
            .get(&v3)
            .unwrap()
            .intersection(vertices.get(&v4).unwrap())
            .find(|&&x| x != quad)
            .map(|x| *x);
        let n4 = vertices
            .get(&v4)
            .unwrap()
            .intersection(vertices.get(&v1).unwrap())
            .find(|&&x| x != quad)
            .map(|x| *x);
        map.insert(quad, [n1, n2, n3, n4]);
    }

    map
}

pub fn get_vertices(path: &str, linear_levels: u32, loop_levels: u32, creases: bool) -> Vec<RVertex> {
    let mut quads = loader::load_wavefront(path);
    quads = divider::linear_subdivide(quads, linear_levels);
    quads = divider::catmull_subdivide(quads, loop_levels, creases);
    transform::transform(quads)
}
