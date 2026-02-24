
pub mod model;
pub mod parser;
pub mod render;
pub mod project;
pub mod consts;

use std::fs;
use std::thread;
use std::time::Duration;

use macroquad::window::Conf;
use macroquad::color::Color;
use macroquad::time::draw_fps;
use macroquad::input::KeyCode;
use macroquad::window::next_frame;
use macroquad::window::clear_background;
use macroquad::input::is_key_pressed;

use crate::model::Vec3;


#[macroquad::main(window_conf)]
async fn main() {
    let path = "dragon.obj";
    // let path = "cube-2.obj";
    let obj_str = fs::read_to_string(path).unwrap();

    let mut polygon = parser::parse_polygon(&obj_str).unwrap();
    let start_loc = Vec3::new(0., 0., 2.5);
    polygon.translation.add(&start_loc);
    let mut polys = vec![polygon];

    loop {
        if is_key_pressed(KeyCode::Q) {
            break;
        }

        let bg_color = Color::from_rgba(32, 32, 32, 255);
        clear_background(bg_color);
        draw_fps();

        // let m_v = Vec3::new(0., 0., 0.02);

        // let m_v = Vec3::new(0., 0., -0.02);
        // polys.iter_mut().for_each(|p| p.translate(&m_v));
        // polys.iter_mut().for_each(|p| p.translation.add(&m_v));
        polys.iter_mut().for_each(|p| p.rotate_y(0.01));

        for p in &polys {
            // render::draw_defined_wireframe(p);
            // render::draw_vertices(p);
            render::draw_defined_faces(p);
            // render::draw_faces(p);
        }

        next_frame().await;
        thread::sleep(Duration::from_millis(10));
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Simple 3D".to_string(),
        window_width: consts::WIDTH,
        window_height: consts::HEIGHT,
        fullscreen: false,
        ..Default::default()
    }
}

