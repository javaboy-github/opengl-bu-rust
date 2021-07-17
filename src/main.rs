#[macro_use]
extern crate glium;
extern crate image;

#[path = "./teapot.rs"]
mod teapot;

#[path = "./camera.rs"]
mod camera;

use cgmath::Vector3;


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

    let mut camera = camera::CameraState::new();
    camera.set_position((2.0, -1.0, 0.0));
    camera.set_direction((-2.0, 1.0, 1.0));

    event_loop.run(move |event, _, control_flow| {
        camera.update();
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                glutin::event::WindowEvent::KeyboardInput {..} => {
                    camera.process_input(&event);
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
            view: camera.get_view(),
            u_light: [-1.0, 0.4, 0.9f32],
            perspective: camera.get_perspective(),
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
