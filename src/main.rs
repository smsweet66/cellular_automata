mod cell_grid;
mod vertex_array;
mod vertex_buffer;
mod vertex_buffer_layout;
mod index_buffer;
mod shader;
mod renderer;

extern crate sdl2;
use std::time::SystemTime;
use gl33::GL_COLOR_BUFFER_BIT;
use gl33::global_loader::{glClear, load_global_gl};
use nalgebra_glm::{Mat4, ortho};
use sdl2::event::Event;
use sdl2::pixels::PixelFormatEnum;
use sdl2::video::DisplayMode;
use sdl2::VideoSubsystem;
use crate::cell_grid::CellGrid;
use crate::shader::Shader;

fn main()
{
    let screen_width = 2560;
    let screen_height = 1440;
    let cols = screen_width/5;
    let rows = screen_height/5;
    let hertz = 120.0;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    video_subsystem.gl_set_swap_interval(1);
    let mut window = video_subsystem.window("rust-sdl2 demo", 2560, 1440)
        .fullscreen_desktop()
        .opengl()
        .build()
        .unwrap();

    let context = window.gl_create_context().unwrap();
    window.gl_make_current(&context);

    unsafe {
        load_global_gl(&|ptr| {
            let c_str = std::ffi::CStr::from_ptr(ptr as *const i8);
            let r_str = c_str.to_str().unwrap();
            video_subsystem.gl_get_proc_address(r_str) as _
        });
    }

    let mut shader = Shader::new();
    shader.bind();

    let proj:Mat4 = ortho(0.0, screen_width as f32, 0.0, screen_height as f32, -1.0, 1.0);
    shader.set_uniform_mat4(&String::from("u_proj"), &proj);

    let mut grid = CellGrid::new(rows as i32, cols as i32, screen_width as f32, screen_height as f32);
    unsafe { glClear(GL_COLOR_BUFFER_BIT) };

    let mut event_pump = sdl_context.event_pump().unwrap();
    'main: loop
    {
        let start = SystemTime::now();
        for event in event_pump.poll_iter()
        {
            match event
            {
                Event::Quit {..}  => { break 'main },
                _ => {}
            }
        }

        unsafe { glClear(GL_COLOR_BUFFER_BIT) };
        grid.update_grid();
        grid.draw_grid(&mut shader);
        window.gl_swap_window();

        while start.elapsed().unwrap().as_secs_f64() < 1.0/hertz {}
    }
}