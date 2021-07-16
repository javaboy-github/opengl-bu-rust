#[macro_use]
extern crate glium;
extern crate image;

#[path = "./teapot.rs"]
mod teapot;

use cgmath::Vector3;

fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    use cgmath::ElementWise;
    /// ベクトル内の要素の合計を返す
    /// # example
    /// ```rust
    /// assert_eq!(sum(Vector3::new(1, 2, 3)), 6);
    /// ```
    fn sum(vec: Vector3<f32>) -> f32 {
        return vec.x + vec.y + vec.z;
    }

    let position = Vector3::new(position[0], position[1], position[2]);
    let direction = Vector3::new(direction[0], direction[1], direction[2]);
    let up = Vector3::new(up[0], up[1], up[2]);

    let f = {
        let f = direction;
        let len = sum(f.map(|e| e.powi(2)));
        let len = len.sqrt();
        f.map(|e| e / len)
    };

    let s = Vector3::new(
        up[1] * f[2] - up[2] * f[1],
        up[2] * f[0] - up[0] * f[2],
        up[0] * f[1] - up[1] * f[0],
    );

    let s_norm = {
        let len = sum(s.map(|e| e.powi(2)));
        let len = len.sqrt();
        s.map(|e| e / len)
    };

    let u = Vector3::new(
        f[1] * s_norm[2] - f[2] * s_norm[1],
        f[2] * s_norm[0] - f[0] * s_norm[2],
        f[0] * s_norm[1] - f[1] * s_norm[0],
    );

    let p = Vector3::new(
        sum(-position.mul_element_wise(s_norm)),
        sum(-position.mul_element_wise(u)),
        sum(-position.mul_element_wise(f)),
    );

    [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}

fn main() {
    #[allow(unused_imports)]
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &teapot::INDICES,
    )
    .unwrap();

    // main.vertを読み込む
    let vertex_shader_src = include_str!("./main.vert");
    // main.fragを読み込む
    let fragment_shader_src = include_str!("./main.frag");

    let program =
        glium::Program::from_source(&display, &vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    // カメラの表示 [位置, 方向, 画面上部のシーン座標の方向]
    // 位置は、ティーポットを中心に置くための値
    // 方向は、シーン座標をカメラが向いている
    // 画面上部のシーン座標の方向は、　スクリーンのトップのシーン座標の方向を表す
    let mut original_view = [[2.0, -1.0, 0.0], [-2.0, 1.0, 1.0], [0.0, 1.0, 0.0]];
    let mut old_cursor_coords = glium::glutin::dpi::PhysicalPosition { x: 0.0, y: 0.0 };
    event_loop.run(move |event, _, control_flow| {
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                glutin::event::WindowEvent::CursorMoved { position, .. } => {
                    let x = position.x;
                    let y = position.y;
                    let dx = old_cursor_coords.x - x; // different x
                    let dy = old_cursor_coords.y - y; // different x
                    original_view[1][2] -= dx as f32 / 10f32;
                    original_view[1][0] -= dy as f32 / 10f32;
                    old_cursor_coords = position;
                }
                glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                    let motion = 1f32;
                }
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0., 1.0, 1.0), 1.0);

        let uniforms = uniform! {
            model: [
                [0.01, 0.0, 0.0, 0.0],
                [0.0, 0.01, 0.0, 0.0],
                [0.0, 0.0, 0.01, 0.0],
                [0.0, 0.0, 2.0, 1.0f32],
            ],
            view: view_matrix(&original_view[0], &original_view[1], &original_view[2]),
            u_light: [-1.0, 0.4, 0.9f32],
            perspective: {
                let (width, height) = target.get_dimensions();
                let aspect_ratio = height as f32 / width as f32;


                let fov: f32 = 3.141592 / 3.0;
                let zfar = 1024.0;
                let znear = 0.1;

                let f = 1.0/ (fov/ 2.0).tan();

                [
                    [f * aspect_ratio, 0.0, 0.0, 0.0],
                    [0.0, f, 0.0, 0.0],
                    [0.0, 0.0, (zfar+znear)/ (zfar-znear), 1.0],
                    [0.0, 0.0, -(2.0*zfar*znear)/(zfar-znear), 0.0]
                ]
            }
        };
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            ..Default::default()
        };

        target
            .draw(
                (&positions, &normals),
                &indices,
                &program,
                &uniforms,
                &params,
            )
            .unwrap();
        target.finish().unwrap();
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// `view_matrix`がしっかり動くかのテスト
    fn does_view_matrix_method_work() {
        assert_eq!(
            view_matrix(&[2.0, -1.0, 0.0], &[-2.0, 1.0, 1.0], &[0.0, 1.0, 0.0]),
            [[0.4472136, 0.36514837, -0.81649655, 0.0],
            [0.0, 0.9128709, 0.40824828, 0.0],
            [0.8944272, -0.18257418, 0.40824828, 0.0],
            [-0.8944272, 0.18257415, 2.0412414, 1.0]],
        );
    }
}
