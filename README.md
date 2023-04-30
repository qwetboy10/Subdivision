# Catmull-Clark and Loop Subdivider
By Tristan Wiesepape (tww679) and Rahul Shanmugham (rss3272)

We both attended the Digital Demo Day, and filled out the eCIS survey

# Build Instructions
Our project is written in rust and requires the rust toolchain to build and run. We have included a prebuilt verison of the project at `target/release/subdivision`, but you can also run the project with `cargo run`. If you use `cargo run` be sure to include a `--` between `cargo run` and the argument list and to run in release mode (i.e. `cargo run --release -- assets/cube.obj`). To build the project simply run `cargo build --release`

# Usage Instructions
We have included several example `.obj` files in the `assets` directory, but the program should work with any `.obj` that is made of triangles or quads. The program takes a single argument, which is the `.obj` file to load. It also has two flags. `-q` tells the program to load a file of quads instead of triangles, and `-c` enables crease detection.

```
Usage: subdivision [OPTIONS] [FILE]

Arguments:
  [FILE]  .obj file to load

Options:
  -c  Keep sharp creases
  -q  Load quads instead of triangles
```

Once the program is running it has the following hotkeys:

0 - Apply 0 levels of linear subdivision

1 - Apply 1 levels of linear subdivision

2 - Apply 2 levels of linear subdivision

3 - Apply 3 levels of linear subdivision


5 - Apply 0 levels of loop / catmull-clark subdivision

6 - Apply 1 levels of loop / catmull-clark subdivision

7 - Apply 2 levels of loop / catmull-clark subdivision

8 - Apply 3 levels of loop / catmull-clark subdivision


Note that if both linear and loop / catmull-clark levels are non-zero, the linear subdivision is ran before the loop / catmull-clark


W/A/S/D - Rotate the camera

Up Arrow / Down Arrow - Zoom the camera

C - Enable / disable crease detection


We have included the following `.obj` files in the assets folder:

- Triangle Meshes
    - cube.obj
    - half_sphere.obj
    - teapot.obj
    - triangle.obj
    - triangular_pyramid.obj
- Quad Meshes
    - quad_cube.obj 
    - quad_half_sphere.obj
    - quad_pentagonal_prism.obj 

# Artifacts
The `photos` folder contains images of the tool being run.
