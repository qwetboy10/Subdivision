use crate::quad_subdivide::{get_adjacency, Quad};
use crate::render::Vertex;
use std::collections::HashMap;
use vecmath::*;

pub fn transform(quads: Vec<Quad>) -> Vec<Vertex> {
    let map = get_adjacency(&quads);
    let mut color_map: HashMap<&Quad, usize> = HashMap::new();
    let colors = [
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 1.0, 0.0],
        [0.0, 1.0, 1.0],
    ];

    let mut av = [0.0, 0.0, 0.0];

    let mut r: Vec<Vertex> = Vec::new();
    let mut _ind = 0;
    for quad in &quads[..] {
        let mut taken = [false, false, false, false, false];
        for n in map.get(quad).unwrap() {
            if let Some(neighbor) = n {
                if color_map.contains_key(neighbor) {
                    taken[*color_map.get(neighbor).unwrap()] = true;
                }
            }
        }

        let mut c = 6;
        for i in 0..5 {
            if !taken[i] {
                c = i;
                break;
            }
        }

        color_map.insert(quad, c);
        // let c = ind % 5;

        let render_vertex1 = Vertex {
            position: quad.vertices[0].position,
            color: colors[c],
        };
        let render_vertex2 = Vertex {
            position: quad.vertices[1].position,
            color: colors[c],
        };
        let render_vertex3 = Vertex {
            position: quad.vertices[2].position,
            color: colors[c],
        };
        let render_vertex4 = Vertex {
            position: quad.vertices[3].position,
            color: colors[c],
        };
        r.push(render_vertex1);
        r.push(render_vertex2);
        r.push(render_vertex3);

        r.push(render_vertex3);
        r.push(render_vertex1);
        r.push(render_vertex4);
        _ind += 1;

        for i in 0..4 {
            av = vec3_add(av, quad.vertices[i].position);
        }
    }

    av = vec3_scale(av, 1.0 / (4.0 * quads.len() as f32));

    for i in 0..r.len() {
        r[i].position = vec3_sub(r[i].position, av);
    }

    r
}
