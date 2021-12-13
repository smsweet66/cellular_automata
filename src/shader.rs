use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::os::raw::{c_int, c_uint};
use std::path::Path;
use std::ptr::null;
use gl33::{GL_COMPILE_STATUS, GL_FRAGMENT_SHADER, GL_INFO_LOG_LENGTH, GL_VERTEX_SHADER, GLenum};
use gl33::global_loader::{glAttachShader, glCompileShader, glCreateProgram, glCreateShader, glDeleteProgram, glDeleteShader, glGetShaderInfoLog, glGetShaderiv, glGetUniformLocation, glLinkProgram, glShaderSource, glUniform1i, glUniform3f, glUniform4f, glUniformMatrix4fv, glUseProgram, glValidateProgram};
use nalgebra_glm::Mat4;

pub(crate) struct Shader
{
    id: u32,
    uniform_location_cache: HashMap<String, i32>
}

impl Shader
{
    pub fn new() -> Self
    {
        let mut vertex_file  = match File::open(Path::new("resources/shaders/vertex.shader")) {
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

        return Self{ id: Shader::create_shader(&vertex_source, &fragment_source), uniform_location_cache: HashMap::new() }
    }

    fn create_shader(vertex_source: &String, fragment_source: &String) -> u32
    {
        let program = glCreateProgram();
        let vs = Shader::compile_shader(GL_VERTEX_SHADER, vertex_source);
        let fs = Shader::compile_shader(GL_FRAGMENT_SHADER, fragment_source);

        glAttachShader(program, vs);
        glAttachShader(program, fs);
        glLinkProgram(program);
        unsafe { glValidateProgram(program) }
        glDeleteShader(vs);
        glDeleteShader(fs);

        return program;
    }

    fn compile_shader(shader_type: GLenum, shader_source: &String) -> u32
    {
        let id:c_uint = glCreateShader(shader_type);
        let src = shader_source.as_bytes();
        unsafe { glShaderSource(id, 1, &src.as_ptr(), null()) }
        glCompileShader(id);

        let mut result: c_int = 0;
        unsafe { glGetShaderiv(id, GL_COMPILE_STATUS, &mut result as *mut c_int) }
        if result == 0
        {
            let mut length: c_int = 0;
            unsafe { glGetShaderiv(id, GL_INFO_LOG_LENGTH, &mut length as *mut c_int) }
            let mut message = String::with_capacity(length as usize);
            unsafe { glGetShaderInfoLog(id, length, &mut length as *mut c_int, message.as_mut_ptr()) }
            print!("Failed to compile ");
            if shader_type == GL_VERTEX_SHADER { print!("vertex") }
            else { print!("fragment") }
            print!(" shader\n");
            println!("{}", message);
            glDeleteShader(id);

            return 0;
        }

        return id;
    }

    pub fn bind(&self)
    { glUseProgram(self.id) }

    pub fn unbind()
    { glUseProgram(0) }

    pub fn set_uniform1i(&mut self, name: &String, value: i32)
    { unsafe { glUniform1i(self.get_uniform_location(name), value) } }

    pub fn set_uniform3f(&mut self, name: &String, v1: f32, v2: f32, v3: f32)
    { unsafe { glUniform3f(self.get_uniform_location(name), v1, v2, v3) } }

    pub fn set_uniform4f(&mut self, name: &String, v1: f32, v2: f32, v3: f32, v4: f32)
    { unsafe { glUniform4f(self.get_uniform_location(name), v1, v2, v3, v4) } }

    pub fn set_uniform_mat4(&mut self, name: &String, matrix: &Mat4)
    { unsafe { glUniformMatrix4fv(self.get_uniform_location(name), 1, 0, matrix.as_ptr()) } }

    fn get_uniform_location(&mut self, name: &String) -> i32
    {
        if self.uniform_location_cache.contains_key(name)
        { return *self.uniform_location_cache.get(name).unwrap() }

        unsafe
        {
            let location: i32 = glGetUniformLocation(self.id, name.as_ptr());
            if location == -1
            { println!("Shader Variable {} does not exist", name) }
            self.uniform_location_cache.insert(name.to_string(), location);

            return location;
        }
    }
}

impl Drop for Shader
{
    fn drop(&mut self)
    { glDeleteProgram(self.id) }
}