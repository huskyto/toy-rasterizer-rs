

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
    pub fn one() -> Self{
        Self::new(1., 1., 1.)
    }
    pub fn add(&mut self, v: &Vec3) {
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;
    }
    pub fn set(&mut self, x: f32, y: f32, z: f32) {
        self.x = x;
        self.y = y;
        self.z = z;
    }
    pub fn len2(&self) -> f32 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }
    pub fn len(&self) -> f32 {
        f32::sqrt(self.len2())
    }
    pub fn unit(&self) -> Vec3 {
        let ln = self.len();
        Vec3::new(self.x / ln, self.y / ln, self.z / ln)
    }
    pub fn dot(&self, b: &Vec3) -> f32 {
        (self.x * b.x) + (self.y * b.y) + (self.z * b.z)
    }
    pub fn mult(&self, s: f32) -> Vec3 {
        Vec3::new(self.x * s, self.y * s, self.z * s)
    }
}

#[derive(Debug, Clone)]
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
    pub fn sub(&self, b: &Vec2) -> Vec2 {
        Vec2::new(self.x - b.x, self.y - b.y)
    }
    pub fn div(&self, s: f32) -> Vec2 {
        Vec2::new(self.x / s, self.y / s)
    }
    pub fn cross(&self, b: &Vec2) -> f32 {
        (self.x * b.y) - (self.y * b.x)
    }
}

#[derive(Debug, Clone)]
pub struct Triangle {
    pub points: [Vec3; 3]
}
impl Triangle {
    pub fn new(p1: Vec3, p2: Vec3, p3: Vec3) -> Self {
        Triangle { points: [p1, p2, p3] }
    }
    pub fn from(point_v: &Vec<Vec3>) -> Option<Self> {
        if point_v.len() != 3 { None }
        else { Some(Self::new(
            point_v[0].clone(), point_v[1].clone(), point_v[2].clone())) }
    }
    pub fn normal(&self) -> Vec3 {
        let v1 = &self.points[0];
        let v2 = &self.points[1];
        let v3 = &self.points[2];

        let a = Vec3::new(v2.x - v1.x, v2.y - v1.y, v2.z - v1.z);
        let b = Vec3::new(v3.x - v1.x, v3.y - v1.y, v3.z - v1.z);

        let nx = (a.y * b.z) - (a.z * b.y);
        let ny = (a.z * b.x) - (a.x * b.z);
        let nz = (a.x * b.y) - (a.y * b.x);

        Vec3::new(nx, ny, nz)
    }
}


#[derive(Debug, Clone)]
pub struct Mesh {
    pub points: Vec<Vec3>,
    pub faces: Vec<Face>,
    pub origin: Vec3,
    pub translation: Vec3,
    pub rotated: Vec<Vec3>,
    pub scale: Vec3
}
impl Mesh {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            faces: Vec::new(),
            origin: Vec3::zero(),
            translation: Vec3::zero(),
            rotated: Vec::new(),
            scale: Vec3::one(),
        }
    }
    pub fn with_points(points: Vec<Vec3>) -> Self {
        Self {
            faces: Vec::new(),
            origin: Vec3::zero(),
            translation: Vec3::zero(),
            rotated: points.clone(),
            scale: Vec3::one(),
            points,
        }
    }
    pub fn with_points_and_faces(points: Vec<Vec3>, faces: Vec<Face>) -> Self {
        Self {
            faces,
            origin: Vec3::zero(),
            translation: Vec3::zero(),
            rotated: points.clone(),
            scale: Vec3::one(),
            points,
        }
    }
    pub fn transformed_vertices(&self) -> Vec<Vec3> {
        let mut t_v = self.rotated.iter()
                .map(|p| p.clone())
                .collect::<Vec<Vec3>>();

        t_v.iter_mut()
                .for_each(|p| p.add(&self.translation));

        t_v
    }

    pub fn rotate_y(&mut self, th: f32) {
        self.rotated.iter_mut().for_each(|point| {
            let cos = f32::cos(th);
            let sin = f32::sin(th);
            let r_x = cos * point.x + sin * point.z;
            let r_y = point.y;
            let r_z = - sin * point.x + cos * point.z;

            point.set(r_x, r_y, r_z);
        });
    }

    pub fn rotate_x(&mut self, th: f32) {
        self.rotated.iter_mut().for_each(|point| {
            let cos = f32::cos(th);
            let sin = f32::sin(th);
            let r_x = point.x;
            let r_y = cos * point.y - sin * point.z;
            let r_z = sin * point.y + cos * point.z;

            point.set(r_x, r_y, r_z);
        });
    }

    pub fn rotate_z(&mut self, th: f32) {
        self.rotated.iter_mut().for_each(|point| {
            let cos = f32::cos(th);
            let sin = f32::sin(th);
            let r_x = cos * point.x - sin * point.y;
            let r_y = sin * point.x + cos * point.y;
            let r_z = point.z;

            point.set(r_x, r_y, r_z);
        });
    }
}

impl Default for Mesh {
    fn default() -> Self {
        Mesh::new()        
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
