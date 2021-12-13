use std::os::raw::c_int;
use std::ptr::null;
use gl33::{GL_TRIANGLES, GL_UNSIGNED_INT};
use gl33::global_loader::glDrawElements;
use crate::index_buffer::IndexBuffer;
use crate::shader::Shader;
use crate::vertex_array::VertexArray;

pub(crate) fn draw(va: &VertexArray, ib: &IndexBuffer, shader: &Shader)
{
    shader.bind();
    va.bind();
    ib.bind();
    unsafe { glDrawElements(GL_TRIANGLES, ib.count as c_int, GL_UNSIGNED_INT, null()) }
}