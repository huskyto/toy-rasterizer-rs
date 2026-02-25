
use std::f32;
use std::f32::INFINITY;

use crate::project;
use crate::model::Mesh;
use crate::model::Vec2;
use crate::model::Vec3;
use crate::model::Triangle;
use crate::consts::VERT_SIZE;
use crate::consts::VERT_COLOR;
use crate::consts::SEG_COLOR;
use crate::consts::SEG_THICKNESS;




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

    pub fn draw_vertices(&mut self, mesh: &Mesh) {
        for t_v in &mesh.transformed_vertices() {
            if let Some(v) = project::val_proj_scrn_vertex(t_v) {
                self.draw_circle(&v, VERT_SIZE, VERT_COLOR);
            }
        }
    }


    pub fn draw_defined_wireframe(&mut self, mesh: &Mesh) {
        let verts = &mesh.transformed_vertices();
        for face in &mesh.faces {
            if face.points.len() > 1 {
                for i in 0..face.points.len() - 1 {
                    if let Some((v1, v2)) = project::val_project_segment(
                            &verts[face.points[i] as usize],
                            &verts[face.points[i + 1] as usize]) {
                        self.draw_line(&v1, &v2, SEG_THICKNESS, SEG_COLOR);
                    }
                }
                if let Some((v1, v2)) = project::val_project_segment(
                            &verts[face.points[face.points.len() - 1] as usize],
                            &verts[face.points[0] as usize]) {
                    self.draw_line(&v1, &v2, SEG_THICKNESS, SEG_COLOR);
                }
            }
        }
    }

    pub fn draw_defined_faces(&mut self, mesh: &Mesh) {
        let verts = &mesh.transformed_vertices();
        let sun_light = Vec3::new(-0.5, 1., -0.5).mult(1.);

        for face in &mesh.faces {
            if face.points.len() >= 3 {
                let r_i = face.points[0];
                let root = &verts[r_i as usize];
                for i in 1..face.points.len() - 1 {
                    let p2 = &verts[face.points[i] as usize];
                    let p3 = &verts[face.points[i + 1] as usize];

                    let triag = Triangle::new(
                                root.clone(), p2.clone(), p3.clone());
                    let normal = triag.normal().unit();
                    if normal.z < 0. {
                        // let r = if normal.x < 0. { 0. } else { normal.x * 255.} as u32;
                        // let g = if normal.y < 0. { 0. } else { normal.y * 255.} as u32;
                        // let b = if normal.z < 0. { 0. } else { normal.z * 255.} as u32;
                        // let r = ((normal.x + 1.) * 127.) as u32;
                        // let g = ((normal.y + 1.) * 127.) as u32;
                        // let b = ((normal.z + 1.) * 127.) as u32;
                        // let c= (r << 16) + (g << 8) + b;

                        let brightness = sun_light.dot(&normal);
                        let clamped = brightness.min(1.).max(0.);
                        let r = (clamped * 127.) as u32;
                        let g = (clamped * 127.) as u32;
                        let b = (clamped * 255.) as u32;
                        let c= (r << 16) + (g << 8) + b;

                        self.draw_3d_triangle(root, p2, p3, c);
                    }
                }
            }
            else {
                println!("Invalid Polygon");
            }
        }
    }

    fn draw_circle(&mut self, pos: &Vec2, size: f32, color: u32) {
        let size_sqr = size * size;
        for x in (pos.x - size) as usize..=(pos.x + size) as usize {
            for y in (pos.y - size) as usize..=(pos.y + size) as usize {
                let d_x = pos.x - x as f32;
                let d_y = pos.y - y as f32;
                let len2 = (d_x * d_x) + (d_y * d_y);
                if len2 <= size_sqr {
                    self.paint_pixel(x, y, color);
                }
            }
        }
    }

    pub fn draw_line(&mut self, a: &Vec2, b: &Vec2, thickness: f32, color: u32) {
        let dx = b.x - a.x;
        let dy = b.y - a.y;

        let normal = Vec2::new(-dy, dx);
        let thick_v = normal.unit().mult(thickness * 0.5);

        let p0 = a.add(&thick_v);
        let p1 = a.sub(&thick_v);
        let p2 = b.add(&thick_v);
        // let p3 = b.sub(&thick_v);

        self.draw_triangle(&p0, &p1, &p2, color);
        // self.draw_triangle(&p2, &p1, &p3, color);
    }

    fn draw_3d_triangle(&mut self, a: &Vec3, b: &Vec3, c: &Vec3, color: u32) {
        if let Some((pa, pb, pc)) = project::val_project_triangle(&a, &b, &c) {
                // Bounding box.
            let min_x = pa.x.min(pb.x).min(pc.x) as usize;
            let max_x = pa.x.max(pb.x).max(pc.x) as usize;
            let min_y = pa.y.min(pb.y).min(pc.y) as usize;
            let max_y = pa.y.max(pb.y).max(pc.y) as usize;

            for x in min_x..=max_x { for y in min_y..=max_y {
                let p = Vec2::new(x as f32, y as f32);
                if let Some((u, v, w)) = Self::barycentric(&pa, &pb, &pc, &p) {
                    if u >= 0.0 && v >= 0.0 && w >= 0.0 {
                        let z = (a.z * u) + (b.z * v) + (c.z * w);
                        self.paint_pixel_z(x, y, z, color);
                    }
                }
            }}
        }
    }

    fn draw_triangle(&mut self, a: &Vec2, b: &Vec2, c: &Vec2, color: u32) {
            // Bounding box.
        let min_x = a.x.min(b.x).min(c.x) as usize;
        let max_x = a.x.max(b.x).max(c.x) as usize;
        let min_y = a.y.min(b.y).min(c.y) as usize;
        let max_y = a.y.max(b.y).max(c.y) as usize;

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                let p = Vec2::new(x as f32, y as f32);
                if let Some((u, v, w)) = Self::barycentric(a, b, c, &p) {
                    if u >= 0.0 && v >= 0.0 && w >= 0.0 {
                        self.paint_pixel(x, y, color);
                    }
                }
            }
        }
    }

    fn get_idx(&self, x: usize, y: usize) -> usize {
        (self.width * y) + x
    }
    fn paint_pixel(&mut self, x: usize, y: usize, color: u32) {
        let idx = self.get_idx(x, y);
        if self.is_point_in_buffer(x, y) {
            self.buffer[idx] = color;
        }
    }
    fn paint_pixel_z(&mut self, x: usize, y: usize, z: f32, color: u32) {
        let idx = self.get_idx(x, y);
        if self.is_point_in_buffer(x, y) {
            let c_z = self.z_buffer[idx];
            if z < c_z {
                self.buffer[idx] = color;
                self.z_buffer[idx] = z;
            }
        }
    }
    fn is_point_in_buffer(&self, x: usize, y: usize) -> bool {
        x > 0 && x < self.width 
            && y > 0 && y < self.height
    }

    fn barycentric(a: &Vec2, b: &Vec2, c: &Vec2, p: &Vec2) -> Option<(f32, f32, f32)> {
        let ab = b.sub(a);
        let ac = c.sub(a);
        let ap = p.sub(a);

        let denom = ab.cross(&ac);
        if denom.abs() < 0.000001 { return None }

        let u = ab.cross(&ap) / denom;
        let v = ap.cross(&ac) / denom;
        let w = 1.0 - u - v;

        Some((u, v, w))
    }
}
