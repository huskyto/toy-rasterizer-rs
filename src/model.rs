

#[derive(Debug, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}
impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {x, y, z}
    }
    pub fn zero() -> Self{
        Self::new(0., 0., 0.)
    }
    pub fn add(&mut self, v: &Vec3) {
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;
    }
}

#[derive(Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self {x, y}
    }
    pub fn zero() -> Self{
        Self::new(0., 0.)
    }
}

#[derive(Debug, Clone)]
pub struct Polygon {
    pub points: Vec<Vec3>,
    pub fill: bool,
    pub faces: Option<Vec<Face>>
}
impl Polygon {
    pub fn new() -> Self {
        Self { points: Vec::new(), fill: false, faces: None }
    }
    pub fn with_points(points: Vec<Vec3>) -> Self {
        Self { points, fill: false, faces: None }
    }
    pub fn with_points_and_faces(points: Vec<Vec3>, faces: Vec<Face>) -> Self {
        Self { points, fill: false, faces: Some(faces) }
    }
    pub fn translate(&mut self, t: &Vec3) {
        self.points.iter_mut().for_each(|v| v.add(t));
    }
}

#[derive(Debug, Clone)]
pub struct Face {
    pub points: Vec<u32>,
}
impl Face {
    pub fn new() -> Self {
        Self { points: Vec::new() }
    }
    pub fn with_points(points: Vec<u32>) -> Self {
        Self { points }
    }
}
