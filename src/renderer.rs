
use std::f32;
use std::f32::INFINITY;

use crate::consts::VERT_COLOR;
use crate::consts::VERT_SIZE;
use crate::project;
use crate::model::Mesh;
use crate::model::Vec2;




#[derive(Debug, Clone)]
pub struct Renderer {
    buffer: Vec<u32>,
    z_buffer: Vec<f32>,
    width: usize,
    height: usize,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        let buffer: Vec<u32> = vec![0; width * height];
        let z_buffer: Vec<f32> = vec![INFINITY; width * height];
        Self { buffer, z_buffer, width, height }
    }
    pub fn get_buffer(&self) -> &Vec<u32> {
        &self.buffer
    }

    pub fn clear(&mut self, color: u32) {
        self.fill(color);
        for z in &mut self.z_buffer {
            *z = f32::INFINITY;
        }
    }

    fn fill(&mut self, color: u32) {
        for c in &mut self.buffer {
            *c = color;
        }
    }

    pub fn draw_vertices(&mut self, p: &Mesh) {
        for t_v in &p.transformed_vertices() {
            if let Some(v) = project::val_proj_scrn_vertex(t_v) {
                self.draw_circle(&v, VERT_SIZE, VERT_COLOR);
            }
        }
    }
    fn draw_circle(&mut self, pos: &Vec2, size: f32, color: u32) {
        let size_sqr = size * size;
        for x in (pos.x - size) as usize..(pos.x + size) as usize + 1 {
            for y in (pos.y - size) as usize..(pos.y + size) as usize + 1 {
                let d_x = pos.x - x as f32;
                let d_y = pos.y - y as f32;
                let len2 = (d_x * d_x) + (d_y * d_y);
                if len2 <= size_sqr {
                    self.paint_pixel(x, y, color);
                }
            }
        }
    }

    fn get_idx(&self, x: usize, y: usize) -> usize{
        (self.width * y) + x
    }
    fn paint_pixel(&mut self, x: usize, y: usize, color: u32) {
        let idx = self.get_idx(x, y);
        self.buffer[idx] = color;
    }
}
