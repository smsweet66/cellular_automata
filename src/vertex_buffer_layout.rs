use std::any::{TypeId, type_name};
use std::os::raw::{c_int, c_uchar};
use gl33::{GL_FLOAT, GL_UNSIGNED_BYTE, GL_UNSIGNED_INT, GLenum};

pub struct VertexBufferElement
{
    pub element_type: GLenum,
    pub count: u32,
    pub normalized: c_uchar
}

impl VertexBufferElement
{
    pub fn get_size_of_type(element_type: GLenum) -> u32
    {
        match element_type
        {
            GL_FLOAT|GL_UNSIGNED_INT => 4,
            GL_UNSIGNED_BYTE => 1,
            _ => 0
        }
    }
}

pub(crate) struct VertexBufferLayout
{
    pub stride: c_int,
    pub elements: Vec<VertexBufferElement>
}

impl VertexBufferLayout
{
    pub fn new() -> Self
    { return Self { stride: 0, elements: Vec::new() } }

    pub fn push<T: 'static>(&mut self, count: u32, normalized: c_uchar)
    {
        if TypeId::of::<T>() == TypeId::of::<f32>()
        {
            self.elements.push(VertexBufferElement{element_type: GL_FLOAT, count, normalized});
            self.stride += (VertexBufferElement::get_size_of_type(GL_FLOAT) * count) as i32;
        }
        else if TypeId::of::<T>() == TypeId::of::<u32>()
        {
            self.elements.push(VertexBufferElement{element_type: GL_UNSIGNED_INT, count, normalized});
            self.stride += (VertexBufferElement::get_size_of_type(GL_UNSIGNED_INT) * count) as i32;
        }
        else if TypeId::of::<T>() == TypeId::of::<u8>()
        {
            self.elements.push(VertexBufferElement{element_type: GL_UNSIGNED_BYTE, count, normalized});
            self.stride += (VertexBufferElement::get_size_of_type(GL_UNSIGNED_BYTE) * count) as i32;
        }
        else
        { println!("{} is not supported!", type_name::<T>())  }
    }
}