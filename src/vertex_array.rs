use std::ffi::c_void;
use std::os::raw::{c_int, c_uint};
use gl33::global_loader::{glBindVertexArray, glDeleteVertexArrays, glEnableVertexAttribArray, glGenVertexArrays, glVertexAttribPointer};
use crate::vertex_buffer::VertexBuffer;
use crate::vertex_buffer_layout::{VertexBufferElement, VertexBufferLayout};

pub(crate) struct VertexArray
{ id: u32 }

impl VertexArray
{
    pub fn new() -> Self
    {
        let mut id: u32 = 0;
        unsafe { glGenVertexArrays(1, &mut id); }

        return Self { id };
    }

    pub fn add_buffer(&self, vb: &VertexBuffer, vbl: &VertexBufferLayout)
    {
        self.bind();
        vb.bind();

        let mut buffer_offset: u32 = 0;
        for i in 0 .. vbl.elements.len()
        {
            unsafe {
                glEnableVertexAttribArray(i as c_uint);
                glVertexAttribPointer(i as c_uint, vbl.elements[i].count as c_int,
                                      vbl.elements[i].element_type, vbl.elements[i].normalized,
                                      vbl.stride, buffer_offset as *const c_void)
            }

            buffer_offset += vbl.elements[i].count * VertexBufferElement::get_size_of_type(vbl.elements[i].element_type);
        }
    }

    pub fn bind(&self)
    { glBindVertexArray(self.id); }

    pub fn unbind()
    { glBindVertexArray(0); }
}

impl Drop for VertexArray
{
    fn drop(&mut self)
    { unsafe { glDeleteVertexArrays(1, &mut self.id) } }
}