#[macro_use]
extern crate glium;
extern crate image;

#[path = "./teapot.rs"]
mod teapot;

fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [
        up[1] * f[2] - up[2] * f[1],
        up[2] * f[0] - up[2] * f[2],
        up[0] * f[1] - up[1] * f[0],
    ];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [
        f[1] * s_norm[2] - f[2] * s_norm[1],
        f[2] * s_norm[0] - f[0] * s_norm[2],
        f[0] * s_norm[1] - f[1] * s_norm[0],
    ];

    let p = [
        -position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
        -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
        -position[0] * f[0] - position[1] * f[1] - position[2] * u[2],
    ];

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

    let mut view = view_matrix(&[2.0, -1.0, 1.0], &[-2.0, 1.0, 1.0], &[0.0, 1.0, 0.0]);
    event_loop.run(move |event, _, control_flow| {
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                    println!("key event: {:?}", input.virtual_keycode);
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
            view:view,
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
