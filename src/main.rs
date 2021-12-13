mod cell_grid;
mod shader;

extern crate glium;
use glium::*;
use nalgebra_glm::{Mat4, ortho};
use crate::cell_grid::CellGrid;
use crate::glutin::GlProfile;
use crate::glutin::window::Fullscreen;
use crate::shader::gen_shader_program;

fn main()
{
    let screen_width = 2560;
    let screen_height = 1440;
    let cols = screen_width/5;
    let rows = screen_height/5;

    let event_loop = glutin::event_loop::EventLoop::new();
    let window = glutin::window::WindowBuilder::new()
        .with_fullscreen(Option::Some(Fullscreen::Borderless(Option::None)));
    let context = glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_hardware_acceleration(Option::Some(true))
        .with_gl_profile(GlProfile::Core);
    let display = Display::new(window, context, &event_loop).unwrap();
    let shader_program = gen_shader_program(&display);

    let proj:Mat4 = ortho(0.0, screen_width as f32, 0.0, screen_height as f32, -1.0, 1.0);

    let mut grid = CellGrid::new(&display,rows as i32, cols as i32, screen_width as f32, screen_height as f32);

    event_loop.run(move |event, _, control_flow|
    {
        match event
        {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_secs_f32(1.0/60.0);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        grid.update_grid();
        grid.draw_grid(&mut target, &shader_program, &proj);
        target.finish().unwrap();
    });
}