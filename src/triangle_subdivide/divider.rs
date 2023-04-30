use crate::triangle_subdivide::*;
use vecmath::*;

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

pub fn linear_subdivide(triangles: Vec<Triangle>, levels: u32) -> Vec<Triangle> {
    let mut triangles = triangles;
    for _ in 0..levels {
        triangles = linear_subdivide_once(triangles)
    }
    triangles
}

pub fn linear_subdivide_once(triangles: Vec<Triangle>) -> Vec<Triangle> {
    let mut tris: Vec<Triangle> = Vec::new();
    for triangle in &triangles[..] {
        let a = triangle.vertices[0];
        let b = triangle.vertices[1];
        let c = triangle.vertices[2];
        let ab = average(&a, &b);
        let bc = average(&c, &b);
        let ac = average(&a, &c);

        tris.push(Triangle {
            vertices: [a, ab, ac],
        });
        tris.push(Triangle {
            vertices: [b, ab, bc],
        });
        tris.push(Triangle {
            vertices: [c, ac, bc],
        });
        tris.push(Triangle {
            vertices: [ab, bc, ac],
        });
    }

    tris
}

pub fn loop_subdivide(triangles: Vec<Triangle>, levels: u32, creases: bool) -> Vec<Triangle> {
    let mut triangles = triangles;
    for _ in 0..levels {
        triangles = loop_subdivide_once(triangles, creases)
    }
    triangles
}

pub fn loop_subdivide_once(triangles: Vec<Triangle>, creases: bool) -> Vec<Triangle> {
    let map = get_vertex_map(&triangles);
    let mut tris: Vec<Triangle> = Vec::new();
    for triangle in &triangles[..] {
        // even vertices
        let a = triangle.vertices[0];
        let b = triangle.vertices[1];
        let c = triangle.vertices[2];

        let ab = compute_odd(&a, &b, &c, get_furthest_vertex(&a, &b, &triangle, &map), creases);
        let bc = compute_odd(&b, &c, &a, get_furthest_vertex(&b, &c, &triangle, &map), creases);
        let ac = compute_odd(&c, &a, &b, get_furthest_vertex(&c, &a, &triangle, &map), creases);

        let a = compute_even(&a, &map, creases);
        let b = compute_even(&b, &map, creases);
        let c = compute_even(&c, &map, creases);

        tris.push(Triangle {
            vertices: [a, ab, ac],
        });
        tris.push(Triangle {
            vertices: [b, ab, bc],
        });
        tris.push(Triangle {
            vertices: [c, ac, bc],
        });
        tris.push(Triangle {
            vertices: [ab, bc, ac],
        });
    }

    tris
}

fn compute_odd(a: &Vertex, b: &Vertex, c: &Vertex, o: Option<&Vertex>, creases: bool) -> Vertex {
    if let Some(d) = o {
        let mut s = 0.0;
        if creases {
            let shared_edge = vec3_normalized(vec3_sub(a.position, b.position));
            let e1 = vec3_normalized(vec3_sub(a.position, c.position));
            let e2 = vec3_normalized(vec3_sub(a.position, d.position));
            let n1 = vec3_normalized(vec3_cross(shared_edge, e1));
            let n2 = vec3_normalized(vec3_cross(e2, shared_edge));
            s = vec3_dot(n1, n2);
        }
        if creases && s < 0.6 {
            let old_pos = vec3_add(
                vec3_scale(a.position, 1.0 / 2.0),
                vec3_scale(b.position, 1.0 / 2.0),
            );
            Vertex { position: old_pos }
        } else {
            Vertex {
                position: vec3_add(
                    vec3_add(
                        vec3_scale(a.position, 3.0 / 8.0),
                        vec3_scale(b.position, 3.0 / 8.0),
                    ),
                    vec3_add(
                        vec3_scale(c.position, 1.0 / 8.0),
                        vec3_scale(d.position, 1.0 / 8.0),
                    ),
                ),
            }
        }
    } else {
        Vertex {
            position: vec3_add(
                vec3_scale(a.position, 1.0 / 2.0),
                vec3_scale(b.position, 1.0 / 2.0),
            ),
        }
    }
}

fn compute_even(v: &Vertex, map: &HashMap<&Vertex, HashSet<&Triangle>>, creases: bool) -> Vertex {
    let tris = map.get(v).unwrap();
    let mut surrounding: HashSet<Vertex> = HashSet::new();
    let mut surrounding2: HashSet<Vertex> = HashSet::new();
    let mut norms = Vec::new();
    for tri in tris {
        if !surrounding.insert(tri.vertices[0]) {
            surrounding2.insert(tri.vertices[0]);
        }
        if !surrounding.insert(tri.vertices[1]) {
            surrounding2.insert(tri.vertices[1]);
        }
        if !surrounding.insert(tri.vertices[2]) {
            surrounding2.insert(tri.vertices[2]);
        }
        if creases {
            let e1 = vec3_normalized(vec3_sub(tri.vertices[0].position, tri.vertices[1].position));
            let e2 = vec3_normalized(vec3_sub(tri.vertices[1].position, tri.vertices[2].position));
            let n = vec3_normalized(vec3_cross(e1,e2));
            norms.push(n);
        }
    }
    surrounding.remove(v);
    surrounding2.remove(v);

    let n = surrounding.len();
    let k = n as f32;

    if n == 2 {
        let mut p = vec3_scale(v.position, 3.0 / 4.0);
        for vertex in &surrounding {
            p = vec3_add(p, vec3_scale(vertex.position, 1.0 / 8.0));
        }
        return Vertex { position: p };
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

    let beta = if n == 3 { 3.0 / 16.0 } else { 3.0 / (8.0 * k) };
    let mut p = vec3_scale(v.position, 1.0 - k * beta);

    for vertex in surrounding {
        p = vec3_add(p, vec3_scale(vertex.position, beta));
    }

    if creases && s < 0.6 {
        Vertex { position: v.position }
    }
    else {
        Vertex { position: p }
    }
}
