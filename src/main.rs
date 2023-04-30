mod camera;
mod quad_subdivide;
mod render;
mod shaders;
mod support;
mod triangle_subdivide;

use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.contains(&"-h".to_string()) || args.len() == 1 {
        println!("Usage: subdivision [OPTIONS] [FILE]");
        println!("");
        println!("Arguments:");
        println!("  [FILE]  .obj file to load");
        println!("");
        println!("Options:");
        println!("  -c  Keep sharp creases");
        println!("  -q  Load quads instead of triangles");
    }
    else {
        let path = args.iter().find(|x| x.contains(".obj")).expect(".obj file expected");
        let creases = args.contains(&"-c".to_string());
        let quads = args.contains(&"-q".to_string());
        render::render(path.to_string(), quads, creases);
    }
}
