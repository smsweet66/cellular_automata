use std::borrow::Borrow;
use boolvec::BoolVec;
use rand::{Rng, thread_rng};
use std::ffi::c_void;
use glium::*;
use glium::index::PrimitiveType::TrianglesList;
use nalgebra_glm::{Mat4, translate, Vec3, Vec4};

pub(crate) struct CellGrid
{
    rows: i32,
    cols: i32,

    living: BoolVec,
    living_buffer: BoolVec,

    vb: VertexBuffer<Vertex>,
    ib: IndexBuffer<u32>
}

impl CellGrid
{
    pub fn new(display: &Display, num_rows: i32, num_cols: i32, screen_width: f32, screen_height: f32) -> Self
    {
        let mut temp = BoolVec::with_capacity((num_rows * num_cols) as usize);
        let mut temp2 = BoolVec::with_capacity((num_rows * num_cols) as usize);
        for i in 0 .. temp.capacity()
        {
            temp.push(thread_rng().gen_bool(0.5));
            temp2.push(temp.get(i).unwrap());
        }

        implement_vertex!(Vertex, position);
        let vb = VertexBuffer::new(display, &[
            Vertex { position: [0.0, screen_height, 1.0, 1.0] },
            Vertex { position: [0.0, screen_height - 3.0, 1.0, 1.0] },
            Vertex { position: [3.0, screen_height, 1.0, 1.0] },
            Vertex { position: [3.0, screen_height - 3.0, 1.0, 1.0] }
        ]).unwrap();

        return Self {
            rows: num_rows, cols: num_cols,
            living: temp, living_buffer: temp2,
            vb, ib: IndexBuffer::new(display, TrianglesList, &[0, 1, 2, 1, 2, 3]).unwrap(),
        };
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

    pub fn draw_grid(&mut self, target: &mut Frame, shader_program: &Program, proj: &Mat4)
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
                    let u_model: [[f32; 4]; 4] = model.into();
                    let u_color: [f32; 4] = Vec4::new(x_pos, y_pos, 15.0, 1.0).into();
                    let u_proj: [[f32; 4]; 4] = (*proj).into();
                    let uniforms = uniform! { u_model: u_model, u_color: u_color , u_proj: u_proj};
                    target.draw(&self.vb, &self.ib, shader_program, &uniforms, &DrawParameters::default()).unwrap();
                }
            }
        }
    }
}

#[derive(Copy, Clone)]
struct Vertex
{ position: [f32; 4] }