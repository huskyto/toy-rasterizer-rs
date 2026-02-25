
pub mod model;
pub mod parser;
pub mod render;
pub mod project;
pub mod consts;
pub mod renderer;

use std::fs;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use minifb::Window;
use minifb::WindowOptions;

use crate::consts::BG_COLOR;
use crate::consts::HEIGHT;
use crate::consts::WIDTH;
use crate::consts::WINDOW_TITLE;
use crate::model::Vec3;
use crate::renderer::Renderer;


fn main() {
    let path = "dragon.obj";
    // let path = "cube.obj";
    // let path = "cube-2.obj";
    // let path = "suzanne.obj";
    let obj_str = fs::read_to_string(path).unwrap();

    let mut polygon = parser::parse_polygon(&obj_str).unwrap();
    let start_loc = Vec3::new(0., 0., 2.5);
    polygon.translation.add(&start_loc);
    let mut polys = vec![polygon];

    // let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new(WINDOW_TITLE,
            WIDTH, HEIGHT, WindowOptions::default())
            .expect("Failed to create window!");

    let mut renderer = Renderer::new(WIDTH, HEIGHT);

    let fps_alpha = 0.02;
    let mut fps: f64 = 0.;

    while window.is_open() {
        let start_time = Instant::now();
        if window.is_key_pressed(minifb::Key::Q, minifb::KeyRepeat::No) {
            break;
        }

        renderer.clear(BG_COLOR);


        // let m_v = Vec3::new(0., 0., 0.02);

        // polys.iter_mut().for_each(|p| p.translation.add(&m_v));
        polys.iter_mut().for_each(|p| p.rotate_y(0.01));

        for p in &polys {
            // renderer.draw_defined_wireframe(p);
            renderer.draw_vertices(p);
            // renderer.draw_defined_faces(p);
        }

        let _ = window.update_with_buffer(renderer.get_buffer(), WIDTH, HEIGHT);

        thread::sleep(Duration::from_millis(5));

        let duration = start_time.elapsed();

        let new_fps = (1_000_000 / duration.as_micros()) as f64;
        fps = fps * (1.0 - fps_alpha) + new_fps * fps_alpha;

        window.set_title(&format!("{} | {} FPS", WINDOW_TITLE, fps as u32));

    }
}

// fn window_conf() -> Conf {
//     Conf {
//         window_title: "Simple 3D".to_string(),
//         window_width: consts::WIDTH,
//         window_height: consts::HEIGHT,
//         fullscreen: false,
//         ..Default::default()
//     }
// }

