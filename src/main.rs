
pub mod model;
pub mod parser;

use std::fs;
use std::fs::File;

use macroquad::color::Color;
use macroquad::color::GREEN;
use macroquad::color::LIME;
use macroquad::color::RED;
use macroquad::color::BLUE;
use macroquad::color::BLACK;
use macroquad::color::WHITE;
use macroquad::color::YELLOW;
use macroquad::input::KeyCode;
use macroquad::input::is_key_pressed;
use macroquad::prelude::glam;
use macroquad::text::draw_text;
use macroquad::shapes::draw_line;
use macroquad::shapes::draw_circle;
use macroquad::time::draw_fps;
use macroquad::window::Conf;
use macroquad::window::next_frame;
use macroquad::window::clear_background;

use crate::model::Vec2;
use crate::model::Vec3;
use crate::model::Polygon;


#[macroquad::main(window_conf)]
async fn main() {
    let obj_str = fs::read_to_string("cube.obj").unwrap();

    let polygon = parser::parse_polygon(&obj_str).unwrap();
    let mut polys = vec![polygon];

    // let init_verts = vec![
    //     Vec3::new(-0.5, -0.5, 0.75),
    //     Vec3::new(-0.5,  0.5, 0.75),
    //     Vec3::new( 0.5,  0.5, 0.75),
    //     Vec3::new( 0.5, -0.5, 0.75),
    // ];
    // let init_poly = Polygon::with_ponts(init_verts);
    // // let mut verts = vec![ ];
    // let mut polys = vec![];
    // for it in 0..15 {
    //     let off = it as f32 * 1.;
    //     let v_off = Vec3::new(0., 0., off);
    //     let mut p = init_poly.clone();
    //     p.translate(&v_off);
    //     polys.push(p);

    //     // for v in &init_verts {
    //     //     verts.push(Vec3::new(v.x, v.y, v.z + off));
    //     // }
    // }
    loop {
        if is_key_pressed(KeyCode::Q) {
            break;
        }

        clear_background(BLACK);
        draw_fps();

        let m_v = Vec3::new(0., 0., 0.02);

        // let m_v = Vec3::new(0., 0., -0.02);
        polys.iter_mut().for_each(|p| p.translate(&m_v));

        for p in &polys {
            // draw_wireframe(p);
            draw_vertices(p);
            draw_defined_faces(p);
            // draw_faces(p);
        }

        next_frame().await;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Simple 3D".to_string(),
        window_width: 800,
        window_height: 800,
        fullscreen: false,
        ..Default::default()
    }
}

// DRAW UTILS //

fn draw_vertices(p: &Polygon) {
    for v in &p.points {
        let proj = val_project(&v);
        if let Some(p) = proj {
            let sp = to_screen(&p);
            draw_circle(sp.x, sp.y, 1.5, YELLOW);
        }
    }
}

fn draw_wireframe(p: &Polygon) {
    if p.points.len() > 1 {
        for i in 0..p.points.len() - 1 {
            if let Some((v1, v2)) = val_project_segment(
                    &p.points[i], &p.points[i + 1]) {
                draw_line(v1.x, v1.y, v2.x, v2.y, 2.0, LIME);
            }
        }
        if let Some((v1, v2)) = val_project_segment(
                    &p.points[p.points.len() - 1], &p.points[0]) {
            draw_line(v1.x, v1.y, v2.x, v2.y, 2.0, LIME);
        }
    }
    else {
        println!("Invalid Polygon");
    }
}

fn draw_defined_faces(polygon: &Polygon) {
    let verts = &polygon.points;
    if polygon.faces.is_none() {
        return;
    }
    for face in polygon.faces.as_ref().unwrap() {
        if face.points.len() >= 3 {
            let r_i = face.points[0];
            let root = &verts[r_i as usize];
            if let Some(v1) = val_project(&root) {
                let v1 = to_screen(&v1);
                for i in 1..face.points.len() - 1 {
                    if let Some((v2, v3)) = val_project_segment(
                            &verts[face.points[i] as usize],
                            &verts[face.points[i + 1] as usize]) {
                        draw_triangle(&v1, &v2, &v3);
                    }
                }
            }
        }
        else {
            println!("Invalid Polygon");
        }
    }
}

fn draw_faces(p: &Polygon) {
    if p.points.len() >= 3 {
        if let Some(v1) = val_project(&p.points[0]) {
            let v1 = to_screen(&v1);
            for i in 1..p.points.len() - 1 {
                if let Some((v2, v3)) = val_project_segment(
                        &p.points[i], &p.points[i + 1]) {
                    draw_triangle(&v1, &v2, &v3);
                }
            }
        }
    }
    else {
        println!("Invalid Polygon");
    }
}

fn draw_triangle(v1: &Vec2, v2: &Vec2, v3: &Vec2) {
    let gv1 = glam::vec2(v1.x, v1.y);
    let gv2 = glam::vec2(v2.x, v2.y);
    let gv3 = glam::vec2(v3.x, v3.y);
    let color = Color::from_rgba(128, 128, 128, 128);
    macroquad::prelude::draw_triangle(gv1, gv2, gv3, color);
}

fn val_project_segment(v1: &Vec3, v2: &Vec3) -> Option<(Vec2, Vec2)> {
    let opt1 = &val_project(v1);
    let opt2 = &val_project(v2);
    if opt1.is_some() && opt2.is_some() {
        Some((to_screen(opt1.as_ref().unwrap()), to_screen(opt2.as_ref().unwrap())))
    }
    else { None }
}
fn val_project_triangle(v1: &Vec3, v2: &Vec3, v3: &Vec3) -> Option<(Vec2, Vec2, Vec2)> {
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

fn val_project(v: &Vec3) -> Option<Vec2> {
        // avoid div by zero
        // cull behind camera
        // cull out of view
    if v.z < 0.4
            || v.x.abs() > 1.
            || v.y.abs() > 1. {
        None
    }
    else {
        Some(project(v))
    }
}

fn to_screen(v: &Vec2) -> Vec2 {
    let w = 800.;
    let h = 800.;
    let x = (v.x + 1.) / 2.;
    let y = (v.y + 1.) / 2.;

    Vec2::new(x * w, y * h)
}

fn project(v: &Vec3) -> Vec2 {
    let px = v.x / v.z;
    let py = v.y / v.z;
    Vec2::new(px, py)
}
