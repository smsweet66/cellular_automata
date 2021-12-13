use std::borrow::Borrow;
use boolvec::BoolVec;
use rand::{Rng, thread_rng};
use crate::vertex_array::VertexArray;
use crate::index_buffer::IndexBuffer;
use crate::vertex_buffer::VertexBuffer;
use crate::vertex_buffer_layout::VertexBufferLayout;
use std::ffi::c_void;
use nalgebra_glm::{Mat4, translate, Vec3};
use crate::renderer::draw;
use crate::shader::Shader;

pub(crate) struct CellGrid
{
    rows: i32,
    cols: i32,

    positions: [f32; 8],
    indexes: [u32; 6],

    living: BoolVec,
    living_buffer: BoolVec,

    va: VertexArray,
    vb: VertexBuffer,
    ib: IndexBuffer
}

impl CellGrid
{
    pub fn new(num_rows: i32, num_cols: i32, screen_width: f32, screen_height: f32) -> Self
    {
        let mut temp = BoolVec::with_capacity((num_rows * num_cols) as usize);
        let mut temp2 = BoolVec::with_capacity((num_rows * num_cols) as usize);
        for i in 0 .. temp.capacity()
        {
            temp.push(thread_rng().gen_bool(0.5));
            temp2.push(temp.get(i).unwrap());
        }
        let mut grid = Self { rows: num_rows, cols: num_cols,
            positions: [
                0.0, screen_height,
                0.0, screen_height - 4.0,
                4.0, screen_height,
                4.0, screen_height - 4.0
            ],
            indexes: [0, 1, 2, 1, 2, 3],
            living: temp, living_buffer: temp2,
            va: VertexArray::new(), vb: VertexBuffer::new(), ib: IndexBuffer::default()
        };

        grid.ib.add_data(&mut grid.indexes as *mut _ as *mut c_void, 6);
        grid.vb.add_data(&mut grid.positions as *mut _ as *mut c_void, 32);
        let mut vbl = VertexBufferLayout::new();
        vbl.push::<f32>(2, 0);
        grid.va.add_buffer(&grid.vb, &vbl);

        return grid;
    }

    pub fn update_grid(&mut self)
    {
        for i in 0 .. self.rows as i32
        {
            for j in 0 .. self.cols as i32
            {
                let mut sum = 0;
                for r in -1i32 ..= 1
                {
                    for c in -1i32 ..=1
                    {
                        if r == 0 && c == 0
                        { continue }

                        sum += self.living.get((((i + r + self.rows) % self.rows) * self.cols
                            + (j + c + self.cols) % self.cols) as usize).unwrap() as i32;
                    }
                }

                if sum < 2 || sum > 3
                { self.living_buffer.set((i * self.cols + j) as usize, false) }
                else if sum == 3
                { self.living_buffer.set((i * self.cols + j) as usize, true) }
                else
                { self.living_buffer.set((i * self.cols + j) as usize, self.living.get((i * self.cols + j) as usize).unwrap()) }
            }
        }

        for i in 0 .. self.rows*self.cols
        { self.living.set(i as usize, self.living_buffer.get(i as usize).unwrap()) }
    }

    pub fn draw_grid(&mut self, shader: &mut Shader)
    {
        for i in 0 .. self.rows
        {
            for j in 0 .. self.cols
            {
                if self.living.get((i * self.cols + j) as usize).unwrap()
                {
                    let x_pos = j as f32 * 5.0;
                    let y_pos = -i as f32 * 5.0;

                    let mut model = Mat4::identity();
                    model = translate(&model, Vec3::new(x_pos, y_pos, 0.0).borrow());
                    shader.set_uniform4f(&String::from("u_color"), self.positions[0] + x_pos, self.positions[1] + y_pos, -15.0, 1.0);
                    shader.set_uniform_mat4(&String::from("u_model"), &model);
                    draw(&self.va, &self.ib, shader);
                }
            }
        }
    }
}