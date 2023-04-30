use crate::render::Vertex;
use crate::triangle_subdivide::{get_adjacency, Triangle};
use std::collections::HashMap;
use vecmath::*;

pub fn transform(triangles: Vec<Triangle>) -> Vec<Vertex> {
    let map = get_adjacency(&triangles);
    let mut color_map: HashMap<&Triangle, usize> = HashMap::new();
    let colors = [
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 1.0, 0.0],
    ];

    let mut av = [0.0, 0.0, 0.0];
    let mut r: Vec<Vertex> = Vec::new();
    for tri in &triangles[..] {
        let mut taken = [false, false, false, false];
        for triangle in map.get(tri).unwrap() {
            if let Some(t) = triangle {
                if color_map.contains_key(t) {
                    taken[*color_map.get(t).unwrap()] = true;
                }
            }
        }

        let mut c = 5;
        for i in 0..4 {
            if !taken[i] {
                c = i;
                break;
            }
        }

        color_map.insert(tri, c);

        let render_vertex1 = Vertex {
            position: tri.vertices[0].position,
            color: colors[c],
        };
        let render_vertex2 = Vertex {
            position: tri.vertices[1].position,
            color: colors[c],
        };
        let render_vertex3 = Vertex {
            position: tri.vertices[2].position,
            color: colors[c],
        };
        r.push(render_vertex1);
        r.push(render_vertex2);
        r.push(render_vertex3);

        for i in 0..3 {
            av = vec3_add(av, tri.vertices[i].position);
        }
    }

    av = vec3_scale(av, 1.0 / (3.0 * triangles.len() as f32));

    for i in 0..r.len() {
        r[i].position = vec3_sub(r[i].position, av);
    }

    r
}
