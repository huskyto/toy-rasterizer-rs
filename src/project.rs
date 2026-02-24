
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

pub fn val_project(v: &Vec3) -> Option<Vec2> {
        // avoid div by zero
        // cull behind camera
        // cull out of view
    // if false {
    if v.z < 0.4
    //         || v.x.abs() > 1.
    //         || v.y.abs() > 1.
    {
        None
    }
    else {
        Some(project(v))
    }
}

pub fn to_screen(v: &Vec2) -> Vec2 {
    let w = 800.;
    let h = 800.;
    let x = (v.x + 1.) / 2.;
    let y = ((v.y * -1.) + 1.) / 2.;

    Vec2::new(x * w, y * h)
}

fn project(v: &Vec3) -> Vec2 {
    let px = v.x / v.z;
    let py = v.y / v.z;
    Vec2::new(px, py)
}
