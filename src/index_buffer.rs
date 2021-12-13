use std::ffi::c_void;
use gl33::{GL_ELEMENT_ARRAY_BUFFER, GL_STATIC_DRAW};
use gl33::global_loader::{glBindBuffer, glBufferData, glDeleteBuffers, glGenBuffers};

pub(crate) struct IndexBuffer
{
    id: u32,
    pub count: u32
}

impl IndexBuffer
{
    pub fn new(data: *const c_void, count: u32) -> Self
    {
        let mut id: u32 = 0;
        unsafe {
            glGenBuffers(1, &mut id);
            glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, id);
            glBufferData(GL_ELEMENT_ARRAY_BUFFER, (count * 4) as isize, data, GL_STATIC_DRAW);
        }

        Self {id, count}
    }

    pub fn add_data(&mut self, data: *const c_void, count: u32)
    {
        self.count = count;
        self.bind();
        unsafe { glBufferData(GL_ELEMENT_ARRAY_BUFFER, (count*4) as isize, data, GL_STATIC_DRAW); }
    }

    pub fn bind(&self)
    { unsafe { glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, self.id) } }

    pub fn unbind()
    { unsafe { glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, 0) } }
}

impl Default for IndexBuffer
{
    fn default() -> Self
    {
        let mut id: u32 = 0;
        unsafe { glGenBuffers(1, &mut id) }
        return Self { id, count: 0 }
    }
}

impl Drop for IndexBuffer
{
    fn drop(&mut self)
    { unsafe { glDeleteBuffers(1, &mut self.id) } }
}