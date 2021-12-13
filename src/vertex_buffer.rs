use std::ffi::c_void;
use gl33::{GL_ARRAY_BUFFER, GL_STATIC_DRAW};
use gl33::global_loader::{glBindBuffer, glBufferData, glDeleteBuffers, glGenBuffers};

pub(crate) struct VertexBuffer
{ id: u32 }

impl VertexBuffer
{
    pub fn new() -> Self
    {
        let mut id: u32 = 0;
        unsafe { glGenBuffers(1, &mut id); }
        return Self{ id };
    }

    pub fn add_data(&mut self, data: *const c_void, size: u32)
    {
        self.bind();
        unsafe { glBufferData(GL_ARRAY_BUFFER, size as isize, data, GL_STATIC_DRAW) };
    }

    pub fn bind(&self)
    { unsafe { glBindBuffer(GL_ARRAY_BUFFER, self.id) } }

    pub fn unbind()
    { unsafe { glBindBuffer(GL_ARRAY_BUFFER, 0) } }
}

impl Drop for VertexBuffer
{
    fn drop(&mut self)
    { unsafe { glDeleteBuffers(1, &self.id) } }
}