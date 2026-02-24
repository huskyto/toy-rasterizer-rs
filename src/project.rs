
use crate::consts;
use crate::model::Vec2;
use crate::model::Vec3;


pub fn val_project_segment(v1: &Vec3, v2: &Vec3) -> Option<(Vec2, Vec2)> {
    let opt1 = &val_project(v1);
    let opt2 = &val_project(v2);
    if opt1.is_some() && opt2.is_some() {
        Some((to_screen(opt1.as_ref().unwrap()), to_screen(opt2.as_ref().unwrap())))
    }
    else { None }
}

pub fn val_project_triangle(v1: &Vec3, v2: &Vec3, v3: &Vec3) -> Option<(Vec2, Vec2, Vec2)> {
    let opt1 = &val_project(v1);
    let opt2 = &val_project(v2);
    let opt3 = &val_project(v3);
    if opt1.is_some() && opt2.is_some() && opt3.is_some() {
        Some((
            to_screen(opt1.as_ref().unwrap()),
            to_screen(opt2.as_ref().unwrap()),
            to_screen(opt3.as_ref().unwrap())))
    }
    else { None }
}

pub fn val_proj_scrn_vertex(v: &Vec3) -> Option<Vec2> {
    val_project(v).map(|v| to_screen(&v))
}

pub fn val_project(v: &Vec3) -> Option<Vec2> {
        // avoid div by zero
        // cull behind camera
    if v.z < 0.4 {
        None
    }
    else {
        Some(project(v))
    }
}

pub fn to_screen(v: &Vec2) -> Vec2 {
    let w = consts::WIDTH as f32;
    let h = consts::HEIGHT as f32;
    let x = (v.x + 1.) / 2.;
    let y = ((v.y * -1.) + 1.) / 2.;

    Vec2::new(x * w, y * h)
}

fn project(v: &Vec3) -> Vec2 {
    let fov = 1.;
    let px = (v.x / v.z) * fov;
    let py = (v.y / v.z) * fov;
    Vec2::new(px, py)
}
