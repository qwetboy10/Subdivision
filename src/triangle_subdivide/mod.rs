mod divider;
mod loader;
mod transform;

use crate::render::Vertex as RVertex;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use vecmath::*;

#[derive(Debug, Hash)]
pub struct Triangle {
    vertices: [Vertex; 3],
}

impl Eq for Triangle {}

impl PartialEq for Triangle {
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

pub fn get_furthest_vertex<'a>(
    a: &'a Vertex,
    b: &'a Vertex,
    me: &'a Triangle,
    map: &'a HashMap<&'a Vertex, HashSet<&'a Triangle>>,
) -> Option<&'a Vertex> {
    map.get(a)
        .unwrap()
        .intersection(map.get(b).unwrap())
        .find(|&&x| x != me)
        .map(|t| {
            if t.vertices[0] != *a && t.vertices[0] != *b {
                return &t.vertices[0];
            } else if t.vertices[1] != *a && t.vertices[1] != *b {
                return &t.vertices[1];
            } else {
                return &t.vertices[2];
            }
        })
}

/*
pub fn get_adjacent<'a>(
    a: &'a Vertex,
    b: &'a Vertex,
    me: &'a Triangle,
    map: &'a HashMap<&'a Vertex, HashSet<&'a Triangle>>,
) -> &'a Triangle {
    *map.get(a)
        .unwrap()
        .intersection(map.get(b).unwrap())
        .find(|&&x| x != me)
        .unwrap()
}
*/

pub fn get_vertex_map(triangles: &Vec<Triangle>) -> HashMap<&Vertex, HashSet<&Triangle>> {
    let mut vertices: HashMap<&Vertex, HashSet<&Triangle>> = HashMap::new();

    for triangle in triangles {
        for v in &triangle.vertices {
            if !vertices.contains_key(v) {
                vertices.insert(&v, HashSet::new());
            }
            vertices.get_mut(&v).unwrap().insert(triangle);
        }
    }

    vertices
}

pub fn get_adjacency(triangles: &Vec<Triangle>) -> HashMap<&Triangle, [Option<&Triangle>; 3]> {
    let mut map = HashMap::new();

    let mut vertices: HashMap<&Vertex, HashSet<&Triangle>> = HashMap::new();

    for triangle in triangles {
        for v in &triangle.vertices {
            if !vertices.contains_key(v) {
                vertices.insert(&v, HashSet::new());
            }
            vertices.get_mut(&v).unwrap().insert(triangle);
        }
    }

    for triangle in triangles {
        let v1 = triangle.vertices[0];
        let v2 = triangle.vertices[1];
        let v3 = triangle.vertices[2];
        let n1 = vertices
            .get(&v1)
            .unwrap()
            .intersection(vertices.get(&v2).unwrap())
            .find(|&&x| x != triangle)
            .map(|x| *x);
        let n2 = vertices
            .get(&v2)
            .unwrap()
            .intersection(vertices.get(&v3).unwrap())
            .find(|&&x| x != triangle)
            .map(|x| *x);

        let n3 = vertices
            .get(&v3)
            .unwrap()
            .intersection(vertices.get(&v1).unwrap())
            .find(|&&x| x != triangle)
            .map(|x| *x);
        map.insert(triangle, [n1, n2, n3]);
    }

    map
}

pub fn get_vertices(path: &str, linear_levels: u32, loop_levels: u32, creases: bool) -> Vec<RVertex> {
    let mut triangles = loader::load_wavefront(path);
    triangles = divider::linear_subdivide(triangles, linear_levels);
    triangles = divider::loop_subdivide(triangles, loop_levels, creases);
    transform::transform(triangles)
}
