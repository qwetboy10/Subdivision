use crate::quad_subdivide::*;
use vecmath::*;

fn center(q: &Quad) -> Vertex {
    average(
        &average(&q.vertices[0], &q.vertices[1]),
        &average(&q.vertices[2], &q.vertices[3]),
    )
}

fn normal(q: &Quad) -> [f32; 3] {
    vec3_normalized(
        vec3_cross(
        vec3_normalized(vec3_sub(q.vertices[0].position, q.vertices[1].position)),
        vec3_normalized(vec3_sub(q.vertices[1].position, q.vertices[2].position)),
        )
    )
}

fn average(a: &Vertex, b: &Vertex) -> Vertex {
    average_n(a, b, 1.0, 1.0)
}

fn average_n(a: &Vertex, b: &Vertex, x: f32, y: f32) -> Vertex {
    Vertex {
        position: vec3_add(
            vec3_scale(a.position, x / (x + y)),
            vec3_scale(b.position, y / (x + y)),
        ),
    }
}

pub fn linear_subdivide(quads: Vec<Quad>, levels: u32) -> Vec<Quad> {
    let mut quads = quads;
    for _ in 0..levels {
        quads = linear_subdivide_once(quads)
    }
    quads
}

pub fn linear_subdivide_once(q: Vec<Quad>) -> Vec<Quad> {
    let mut quads: Vec<Quad> = Vec::new();
    for quad in &q[..] {
        let a = quad.vertices[0];
        let b = quad.vertices[1];
        let c = quad.vertices[2];
        let d = quad.vertices[3];
        let ab = average(&a, &b);
        let ad = average(&a, &d);
        let cd = average(&c, &d);
        let bc = average(&b, &c);
        let center = average(&ab, &cd);

        quads.push(Quad {
            vertices: [a, ab, center, ad],
        });
        quads.push(Quad {
            vertices: [ab, b, bc, center],
        });
        quads.push(Quad {
            vertices: [center, bc, c, cd],
        });
        quads.push(Quad {
            vertices: [ad, center, cd, d],
        });
    }

    quads
}

pub fn catmull_subdivide(quads: Vec<Quad>, levels: u32, creases: bool) -> Vec<Quad> {
    let mut quads = quads;
    for _ in 0..levels {
        quads = catmull_subdivide_once(quads, creases)
    }
    quads
}

pub fn get_edge(
    a: &Vertex,
    b: &Vertex,
    me: &Quad,
    map: &HashMap<&Vertex, HashSet<&Quad>>,
    creases: bool
) -> Vertex {
    if let Some(neighbor_quad) = get_adjacent(a, b, me, map) {
        let neighbor_center = center(neighbor_quad);
        let my_center = center(me);
        let edge_center = average(a, b);

        let mut s = 0.0;
        if creases {
            let n1 = normal(me);
            let n2 = normal(neighbor_quad);
            s = vec3_dot(n1,n2);
        }

        if creases && s < 0.6 {
            average(a, b)
        }
        else {
            average(&average(&my_center, &neighbor_center), &edge_center)
        }
    }
    else {
        average(a, b)
    }
}

pub fn get_edge_midpoints(a: &Vertex, q: &Quad) -> (Vertex, Vertex) {
    for i in 0..4 {
        if *a == q.vertices[i] {
            let i1 = if i == 0 { 3 } else { i - 1 };
            let i2 = if i + 1 > 3 { 0 } else { i + 1 };
            return (average(a, &q.vertices[i1]), average(a, &q.vertices[i2]));
        }
    }

    panic!("vertex was not part of quad");
}

pub fn compute_original(a: &Vertex, map: &HashMap<&Vertex, HashSet<&Quad>>, creases: bool) -> Vertex {
    let neighbors = map.get(a).unwrap();
    let n = neighbors.len();
    let mut pos = [0.0, 0.0, 0.0];
    let mut norms = Vec::new();
    for quad in neighbors {
        pos = vec3_add(pos, center(quad).position);
        if creases {
            norms.push(normal(quad));
        }
    }

    let mut s = 1.0;
    if creases {
        for i in 0..norms.len() {
            for j in 0..norms.len() {
                if i != j {
                    let ss = vec3_dot(norms[i],norms[j]);
                    if ss < s {
                        s = ss;
                    }
                }
            }
        }
    }

    if creases && s < 0.6 {
        a.clone()
    }
    else {
        let f = vec3_scale(pos, 1.0 / (n as f32));

        let mut pos = [0.0, 0.0, 0.0];
        for quad in neighbors {
            let (p1, p2) = get_edge_midpoints(&a, &quad);
            pos = vec3_add(pos, p1.position);
            pos = vec3_add(pos, p2.position);
        }

        let r = vec3_scale(pos, 1.0 / (n as f32 * 2.0));

        let p = vec3_scale(
            vec3_add(
                vec3_add(f, vec3_scale(r, 2.0)),
                vec3_scale(a.position, n as f32 - 3.0),
            ),
            1.0 / (n as f32),
        );

        Vertex { position: p }
    }
}

pub fn catmull_subdivide_once(quads: Vec<Quad>, creases: bool) -> Vec<Quad> {
    let map = get_vertex_map(&quads);
    let mut r: Vec<Quad> = Vec::new();
    for quad in &quads[..] {
        let a = quad.vertices[0];
        let b = quad.vertices[1];
        let c = quad.vertices[2];
        let d = quad.vertices[3];

        // face point
        let center_p = center(quad);

        let e1 = get_edge(&a, &b, &quad, &map, creases);
        let e2 = get_edge(&b, &c, &quad, &map, creases);
        let e3 = get_edge(&c, &d, &quad, &map, creases);
        let e4 = get_edge(&a, &d, &quad, &map, creases);

        let na = compute_original(&a, &map, creases);
        let nb = compute_original(&b, &map, creases);
        let nc = compute_original(&c, &map, creases);
        let nd = compute_original(&d, &map, creases);

        r.push(Quad {
            vertices: [na, e1, center_p, e4],
        });

        r.push(Quad {
            vertices: [e1, nb, e2, center_p],
        });

        r.push(Quad {
            vertices: [center_p, e2, nc, e3],
        });

        r.push(Quad {
            vertices: [e4, center_p, e3, nd],
        });
    }

    r
}
