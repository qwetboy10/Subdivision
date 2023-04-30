use crate::quad_subdivide::{Quad, Vertex};

use std::fs::File;
use std::io::prelude::*;

pub fn load_wavefront(path: &str) -> Vec<Quad> {
    let mut file = File::open(path).expect("File not found");
    let mut bytes: Vec<u8> = Vec::new();
    file.read_to_end(&mut bytes).unwrap();

    let mut data = ::std::io::BufReader::new(&bytes[..]);
    let data = obj::ObjData::load_buf(&mut data).unwrap();

    let mut quads = Vec::new();

    for object in data.objects.iter() {
        for polygon in object.groups.iter().flat_map(|g| g.polys.iter()) {
            match polygon {
                obj::SimplePolygon(indices) => {
                    assert!(indices.len() == 4);
                    let v1i = indices.get(0).unwrap();
                    let v2i = indices.get(1).unwrap();
                    let v3i = indices.get(2).unwrap();
                    let v4i = indices.get(3).unwrap();

                    let v1 = Vertex {
                        position: data.position[v1i.0],
                    };
                    let v2 = Vertex {
                        position: data.position[v2i.0],
                    };
                    let v3 = Vertex {
                        position: data.position[v3i.0],
                    };
                    let v4 = Vertex {
                        position: data.position[v4i.0],
                    };

                    let quad = Quad {
                        vertices: [v1, v2, v3, v4],
                    };

                    quads.push(quad);
                }
            }
        }
    }

    quads
}
