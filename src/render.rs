use glium::vertex::VertexBufferAny;
use glium::{self, implement_vertex};
use glium::{glutin, program, uniform, Surface};

use crate::camera;
use crate::quad_subdivide;
use crate::shaders;
use crate::support;
use crate::triangle_subdivide;


#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

implement_vertex!(Vertex, position, color);

pub fn render(path: String, quads: bool, mut creases: bool) {
    let vertex_data = if quads {
        quad_subdivide::get_vertices(&path, 0, 0, creases)
    } else {
        triangle_subdivide::get_vertices(&path, 0, 0, creases)
    };

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(1024.0, 768.0))
        .with_title("CS354H Final Project");

    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let program = program!(&display,
        140 => {
            vertex: shaders::VERTEX_SHADER,
            fragment: shaders::FRAGMENT_SHADER_FLAT,
            // fragment: shaders::FRAGMENT_SHADER_SHADED,
        },
    )
    .unwrap();

    let averagex =
        vertex_data.iter().map(|v| v.position[0]).sum::<f32>() / (vertex_data.len() as f32);
    let averagey =
        vertex_data.iter().map(|v| v.position[1]).sum::<f32>() / (vertex_data.len() as f32);
    let minz = vertex_data
        .iter()
        .map(|v| v.position[2])
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let maxz = vertex_data
        .iter()
        .map(|v| v.position[2])
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    let diff = if (maxz - minz) < 1.0 {
        2.0
    } else {
        2.0 * (maxz - minz)
    };

    let mut camera = camera::Camera::new(
        [averagex, averagey, minz - diff],
        [0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
    );

    let mut linear_levels = 0;
    let mut loop_levels = 0;

    let mut vertex_buffer: VertexBufferAny =
        glium::vertex::VertexBuffer::new(&display, &vertex_data)
            .unwrap()
            .into();

    support::start_loop(event_loop, move |events| {
        camera.update();

        let uniforms = uniform! {
            persp_matrix: camera.get_perspective(),
            view_matrix: camera.get_view(),
        };

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        let mut target = display.draw();
        target.clear_color_and_depth((1.0, 1.0, 1.0, 1.0), 1.0);
        target
            .draw(
                &vertex_buffer,
                &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                &program,
                &uniforms,
                &params,
            )
            .unwrap();
        target.finish().unwrap();

        let mut action = support::Action::Continue;

        let mut changed = false;

        for event in events {
            match event {
                glutin::event::Event::WindowEvent { event, .. } => match event {
                    glutin::event::WindowEvent::CloseRequested => action = support::Action::Stop,
                    glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                        let pressed = input.state == glutin::event::ElementState::Pressed;
                        camera.process_input(&input);
                        let o1 = linear_levels;
                        let o2 = loop_levels;
                        let o3 = creases;
                        if pressed {
                            if let Some(key) = input.virtual_keycode {
                                match key {
                                    glutin::event::VirtualKeyCode::Key0 => linear_levels = 0,
                                    glutin::event::VirtualKeyCode::Key1 => linear_levels = 1,
                                    glutin::event::VirtualKeyCode::Key2 => linear_levels = 2,
                                    glutin::event::VirtualKeyCode::Key3 => linear_levels = 3,
                                    glutin::event::VirtualKeyCode::Key5 => loop_levels = 0,
                                    glutin::event::VirtualKeyCode::Key6 => loop_levels = 1,
                                    glutin::event::VirtualKeyCode::Key7 => loop_levels = 2,
                                    glutin::event::VirtualKeyCode::Key8 => loop_levels = 3,
                                    glutin::event::VirtualKeyCode::C => creases = !creases,
                                    _ => (),
                                }
                            }
                        }
                        changed = o1 != linear_levels || o2 != loop_levels || o3 != creases;
                    }
                    _ => (),
                },
                _ => (),
            }
        }

        if changed {
            let vertex_data = if quads {
                quad_subdivide::get_vertices(&path, linear_levels, loop_levels, creases)
            } else {
                triangle_subdivide::get_vertices(&path, linear_levels, loop_levels, creases)
            };
            vertex_buffer = glium::vertex::VertexBuffer::new(&display, &vertex_data)
                .unwrap()
                .into();
        }


        action
    });
}
