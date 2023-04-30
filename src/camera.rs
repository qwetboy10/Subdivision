use glium::glutin;

use crate::camera::glutin::event::KeyboardInput;
use vecmath::*;

// minor portions have been taken from https://github.com/glium/glium/blob/master/examples/support/camera.rs

#[derive(Debug)]
pub struct Camera {
    aspect_ratio: f32,
    eye: Vector3<f32>,
    forward: Vector3<f32>,
    up: Vector3<f32>,
    right: Vector3<f32>,
    dist: f32,
    rotate_h: f32,
    rotate_v: f32,
    zoom: f32,
}

impl Camera {
    pub fn target(&self) -> Vector3<f32> {
        vec3_add(self.eye, vec3_scale(self.forward, self.dist))
    }

    pub fn new(position: Vector3<f32>, target: Vector3<f32>, up: Vector3<f32>) -> Camera {
        let forward = vec3_normalized(vec3_sub(position, target));
        let right = vec3_normalized(vec3_cross(up, forward));
        Camera {
            aspect_ratio: 1024.0 / 768.0,
            eye: position,
            forward,
            right,
            up: vec3_normalized(vec3_cross(forward, right)),
            dist: vec3_len(vec3_sub(position, target)),
            rotate_h: 0.0,
            rotate_v: 0.0,
            zoom: 0.0,
        }
    }

    pub fn get_perspective(&self) -> Matrix4<f32> {
        let fov: f32 = 3.141592 / 2.0;
        let zfar = 1024.0;
        let znear = 0.1;

        let f = 1.0 / (fov / 2.0).tan();

        // note: remember that this is column-major, so the lines of code are actually columns
        [
            [f / self.aspect_ratio, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
            [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
        ]
    }

    pub fn get_view(&self) -> Matrix4<f32> {
        let position = self.eye;
        let target = self.target();
        let up = self.up;

        let z = vec3_normalized(vec3_sub(position, target));
        let x = vec3_normalized(vec3_cross(up, z));
        let y = vec3_normalized(vec3_cross(z, x));

        return [
            [x[0], y[0], z[0], 0.0],
            [x[1], y[1], z[1], 0.0],
            [x[2], y[2], z[2], 0.0],
            [
                -vec3_dot(x, position),
                -vec3_dot(y, position),
                -vec3_dot(z, position),
                1.0,
            ],
        ];
    }

    pub fn rotate(&mut self, axis: Vector3<f32>, radians: f32, pos: Vector3<f32>) {
        let axis = vec3_normalized(axis);

        let x = axis[0];
        let y = axis[1];
        let z = axis[2];

        let s = radians.sin();
        let c = radians.cos();
        let t = 1.0 - c;

        let rot: Matrix3<f32> = [
            [c + x * x * t, y * x * t + z * s, z * x * t - y * s],
            [x * y * t - z * s, c + y * y * t, z * y * t + x * s],
            [x * z * t + y * s, y * z * t - x * s, c + z * z * t],
        ];

        self.up = vec3_normalized(col_mat3_transform(rot, self.up));
        self.forward = vec3_normalized(col_mat3_transform(rot, self.forward));
        self.right = vec3_normalized(col_mat3_transform(rot, self.right));
        let mut diff = vec3_sub(self.eye, pos);
        diff = col_mat3_transform(rot, diff);
        self.eye = vec3_add(pos, diff);
    }

    pub fn update(&mut self) {
        self.rotate(self.up, self.rotate_h * 3.14 / 100.0, [0.0, 0.0, 0.0]);
        self.rotate(self.right, self.rotate_v * 3.14 / 100.0, [0.0, 0.0, 0.0]);

        self.eye = vec3_add(self.eye, vec3_scale(self.forward, 0.1 * self.zoom));
    }

    pub fn process_input(&mut self, input: &KeyboardInput) {
        let pressed = input.state == glutin::event::ElementState::Pressed;
        if let Some(key) = input.virtual_keycode {
            match key {
                glutin::event::VirtualKeyCode::D => self.rotate_h = if pressed { 1.0 } else { 0.0 },
                glutin::event::VirtualKeyCode::A => {
                    self.rotate_h = if pressed { -1.0 } else { 0.0 }
                }
                glutin::event::VirtualKeyCode::W => self.rotate_v = if pressed { 1.0 } else { 0.0 },
                glutin::event::VirtualKeyCode::S => {
                    self.rotate_v = if pressed { -1.0 } else { 0.0 }
                }

                glutin::event::VirtualKeyCode::Up => {
                    self.zoom = if pressed { -1.0 } else { 0.0 }
                }
                glutin::event::VirtualKeyCode::Down => {
                    self.zoom = if pressed { 1.0 } else { 0.0 }
                }
                _ => (),
            };
        };
    }
}
