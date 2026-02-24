
pub mod model;
pub mod parser;
pub mod render;
pub mod project;

use std::fs;

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
    let obj_str = fs::read_to_string("dragon.obj").unwrap();

    let polygon = parser::parse_polygon(&obj_str).unwrap();
    let mut polys = vec![polygon];

    loop {
        if is_key_pressed(KeyCode::Q) {
            break;
        }

        let bg_color = Color::from_rgba(32, 32, 32, 255);
        clear_background(bg_color);
        draw_fps();

        let m_v = Vec3::new(0., 0., 0.02);

        // let m_v = Vec3::new(0., 0., -0.02);
        // polys.iter_mut().for_each(|p| p.translate(&m_v));
        polys.iter_mut().for_each(|p| p.translation.add(&m_v));
        polys.iter_mut().for_each(|p| p.rotate_y(0.01));

        for p in &polys {
            render::draw_defined_wireframe(p);
            // draw_vertices(p);
            // draw_defined_faces(p);
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

