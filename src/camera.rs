extern crate cgmath;
use glium::glutin;
pub struct CameraState {
    aspect_ratio: f32,
    position: (f32, f32, f32),
    //position: cgmath::Vector3<f32>,
    direction: (f32, f32, f32),
    // direction: cgmath::Vector3<f32>
    up: (f32, f32, f32),
    // up: cgmath::Vector3<f32>,

    old_cursor_position: glium::glutin::dpi::PhysicalPosition<f64>,

    moving_up: bool,
    moving_left: bool,
    moving_down: bool,
    moving_right: bool,
    moving_forward: bool,
    moving_backward: bool,
}

impl CameraState {
    pub fn new() -> CameraState {
        CameraState {
            aspect_ratio: 1024.0 / 768.0,
            position: (0.0, 0.0, 0.0),
            direction: (0.0, 0.0, -1.0),
            up: (0.0, 1.0, 0.0),
            old_cursor_position: glutin::dpi::PhysicalPosition { x: 750f64, y: 750f64 },
            moving_up: false,
            moving_left: false,
            moving_down: false,
            moving_right: false,
            moving_forward: false,
            moving_backward: false,
        }
    }

    pub fn set_position(&mut self, pos: (f32, f32, f32)) {
        self.position = pos;
    }

    pub fn set_direction(&mut self, dir: (f32, f32, f32)) {
        self.direction = dir;
    }

    pub fn set_up(&mut self, up: (f32, f32, f32)) {
        self.up = up;
    }

    pub fn get_perspective(&self) -> [[f32; 4]; 4] {
        let fov: f32 = 3.141592 / 2.0;
        let zfar = 1024.0;
        let znear = 0.1;

        let f = 1.0 / (fov / 2.0).tan();

        // NOTE: remember that this is column-major, so the lines of code are actually columns
        [
            [f / self.aspect_ratio, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
            [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
        ]
    }

    pub fn get_view(&self) -> [[f32; 4]; 4] {
        let f = {
            let f = self.direction;
            let len = f.0 * f.0 + f.1 * f.1 + f.2 * f.2;
            let len = len.sqrt();
            (f.0 / len, f.1 / len, f.2 / len)
        };

        let s = (
            f.1 * self.up.2 - f.2 * self.up.1,
            f.2 * self.up.0 - f.0 * self.up.2,
            f.0 * self.up.1 - f.1 * self.up.0,
        );

        // ノルムとは、ベクトルの長さのこと
        let s_norm = {
            let len = s.0 * s.0 + s.1 * s.1 + s.2 * s.2;
            let len = len.sqrt();
            (s.0 / len, s.1 / len, s.2 / len)
        };

        let u = (
            s_norm.1 * f.2 - s_norm.2 * f.1,
            s_norm.2 * f.0 - s_norm.0 * f.2,
            s_norm.0 * f.1 - s_norm.1 * f.0,
        );

        let p = (
            -self.position.0 * s.0 - self.position.1 * s.1 - self.position.2 * s.2,
            -self.position.0 * u.0 - self.position.1 * u.1 - self.position.2 * u.2,
            -self.position.0 * f.0 - self.position.1 * f.1 - self.position.2 * f.2,
        );

        // note: remember that this is column-major, so the lines of code are actually columns
        [
            [s_norm.0, u.0, f.0, 0.0],
            [s_norm.1, u.1, f.1, 0.0],
            [s_norm.2, u.2, f.2, 0.0],
            [p.0, p.1, p.2, 1.0],
        ]
    }

    pub fn update(&mut self) {
        let f = {
            let f = self.direction;
            let len = f.0 * f.0 + f.1 * f.1 + f.2 * f.2;
            let len = len.sqrt();
            (f.0 / len, f.1 / len, f.2 / len)
        };

        let up = self.up;

        let s = (
            f.1 * up.2 - f.2 * up.1,
            f.2 * up.0 - f.0 * up.2,
            f.0 * up.1 - f.1 * up.0,
        );

        let s = {
            let len = s.0 * s.0 + s.1 * s.1 + s.2 * s.2;
            let len = len.sqrt();
            (s.0 / len, s.1 / len, s.2 / len)
        };

        let u = (
            s.1 * f.2 - s.2 * f.1,
            s.2 * f.0 - s.0 * f.2,
            s.0 * f.1 - s.1 * f.0,
        );

        if self.moving_up {
            self.position.0 += u.0 * 0.01;
            self.position.1 += u.1 * 0.01;
            self.position.2 += u.2 * 0.01;
        }

        if self.moving_left {
            self.position.0 -= s.0 * 0.01;
            self.position.1 -= s.1 * 0.01;
            self.position.2 -= s.2 * 0.01;
        }

        if self.moving_down {
            self.position.0 -= u.0 * 0.01;
            self.position.1 -= u.1 * 0.01;
            self.position.2 -= u.2 * 0.01;
        }

        if self.moving_right {
            self.position.0 += s.0 * 0.01;
            self.position.1 += s.1 * 0.01;
            self.position.2 += s.2 * 0.01;
        }

        if self.moving_forward {
            self.position.0 += f.0 * 0.01;
            self.position.1 += f.1 * 0.01;
            self.position.2 += f.2 * 0.01;
        }

        if self.moving_backward {
            self.position.0 -= f.0 * 0.01;
            self.position.1 -= f.1 * 0.01;
            self.position.2 -= f.2 * 0.01;
        }
    }

    pub fn process_input(&mut self, event: &glutin::event::WindowEvent<'_>) {
        use glium::glutin::event::VirtualKeyCode;
        let input = match *event {
            glutin::event::WindowEvent::KeyboardInput { input, .. } => input,
            _ => return,
        };
        let pressed = input.state == glutin::event::ElementState::Pressed;
        let key = match input.virtual_keycode {
            Some(key) => key,
            None => return,
        };
        match key {
            VirtualKeyCode::Space => self.moving_up = pressed,
            VirtualKeyCode::LShift | VirtualKeyCode::RShift => self.moving_down = pressed,
            VirtualKeyCode::A => self.moving_left = pressed,
            VirtualKeyCode::D => self.moving_right = pressed,
            VirtualKeyCode::W => self.moving_forward = pressed,
            VirtualKeyCode::S => self.moving_backward = pressed,
            _ => (),
        };
    }

    pub fn process_cursor(&mut self, event: &glutin::event::WindowEvent<'_>) {
        let position = match *event {
            glutin::event::WindowEvent::CursorMoved { position, .. } => position,
            _ => return,
        };
        self.up.0 -= (self.old_cursor_position.x - position.x) as f32; // x
        self.up.2 -= (self.old_cursor_position.y - position.y) as f32; // z

        self.old_cursor_position = position;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// `view_matrix`がしっかり動くかのテスト
    fn can_calculate_view_matrix() {
        let mut camera = CameraState::new();
        camera.set_position((2.0, -1.0, 0.0));
        camera.set_direction((-2.0, 1.0, 1.0));
        camera.set_up((0.0, 1.0, 0.0));
        assert_eq!(
            camera.get_view(),
            [
                [-0.4472136, 0.36514837, -0.81649655, 0.0],
                [0.0, 0.9128709, 0.40824828, 0.0],
                [-0.8944272, -0.18257418, 0.40824828, 0.0],
                [0.81649655, 0.18257415, 2.0412414, 1.0]
            ],
        );
    }
}
