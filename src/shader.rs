use std::fs::File;
use std::io::Read;
use std::path::Path;
use glium::*;

pub fn gen_shader_program(display: &Display) -> Program
{
    let mut vertex_file = match File::open(Path::new("resources/shaders/vertex.shader")) {
        Err(why) => panic!("couldn't open vertex.shader: {}", why),
        Ok(file) => file
    };

    let mut vertex_source = String::new();
    vertex_file.read_to_string(&mut vertex_source);

    let mut fragment_file = match File::open(Path::new("resources/shaders/fragment.shader")) {
        Err(why) => panic!("couldn't open fragment.shader: {}", why),
        Ok(file) => file
    };

    let mut fragment_source = String::new();
    fragment_file.read_to_string(&mut fragment_source);

    return glium::program::Program::from_source(display, vertex_source.as_str(),
                                                fragment_source.as_str(),
                                                None).unwrap();
}