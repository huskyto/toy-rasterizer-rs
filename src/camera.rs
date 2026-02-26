
use crate::consts;
use crate::model::Vec2;
use crate::model::Vec3;


#[derive(Debug, Clone)]
pub struct Camera {
    pub fov: f32,
    pub position: Vec3,
    pub direction: Vec3
}
impl Camera {
    pub fn val_proj_scrn_vertex(&self, v: &Vec3) -> Option<Vec2> {
        self.val_project(v).map(|v| Self::to_screen(&v))
    }
    pub fn val_project_segment(&self, v1: &Vec3, v2: &Vec3) -> Option<(Vec2, Vec2)> {
        let opt1 = &self.val_project(v1);
        let opt2 = &self.val_project(v2);
        if opt1.is_some() && opt2.is_some() {
            Some((Self::to_screen(opt1.as_ref().unwrap()), Self::to_screen(opt2.as_ref().unwrap())))
        }
        else { None }
    }

    pub fn val_project_triangle(&self, v1: &Vec3, v2: &Vec3, v3: &Vec3) -> Option<(Vec2, Vec2, Vec2)> {
        let opt1 = &self.val_project(v1);
        let opt2 = &self.val_project(v2);
        let opt3 = &self.val_project(v3);
        if opt1.is_some() && opt2.is_some() && opt3.is_some() {
            Some((
                Self::to_screen(opt1.as_ref().unwrap()),
                Self::to_screen(opt2.as_ref().unwrap()),
                Self::to_screen(opt3.as_ref().unwrap())))
        }
        else { None }
    }

    fn val_project(&self, v: &Vec3) -> Option<Vec2> {
        let mut v = v.clone();
        v.sub(&self.position);
            // near-plane cull
        if v.z < 0.2 {
            None
        }
        else {
            let px = (v.x / v.z) * self.fov;
            let py = (v.y / v.z) * self.fov;
            Some(Vec2::new(px, py))
        }
    }

    pub fn to_screen(v: &Vec2) -> Vec2 {
        let w = consts::WIDTH as f32;
        let h = consts::HEIGHT as f32;
        let x = ( v.x + 1.) / 2.;
        let y = (-v.y + 1.) / 2.;

        Vec2::new(x * w, y * h)
    }
}
impl Default for Camera {
    fn default() -> Self {
        Self {
            fov: 1.,
            position: Vec3::zero(),
            direction: Vec3::new(0., 0., 1.) }
    }
}