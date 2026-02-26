
pub mod model;
pub mod camera;
pub mod consts;
pub mod parser;
pub mod renderer;

use std::fs;
use std::thread;
use std::time::Instant;
use std::time::Duration;

use minifb::Window;
use minifb::WindowOptions;

use crate::model::Mesh;
use crate::model::Vec3;
use crate::camera::Camera;
use crate::consts::WIDTH;
use crate::consts::HEIGHT;
use crate::consts::BG_COLOR;
use crate::consts::WINDOW_TITLE;
use crate::renderer::Renderer;


fn main() {
    let path = "dragon.obj";
    // let path = "cube.obj";
    // let path = "cube-2.obj";
    // let path = "suzanne.obj";
    let obj_str = fs::read_to_string(path).unwrap();

    let mut polys = vec![
        spawn_mesh_at(path, 0., 0., 5.),
        spawn_mesh_at(path, -6., -5.,  9.),
        spawn_mesh_at(path, -5.,  7., 10.),
        spawn_mesh_at(path,  5.,  4.,  8.),
        spawn_mesh_at(path,  3., -4., 12.),
        spawn_mesh_at(path, -4., -7., 11.),
        spawn_mesh_at(path, -7.,  5., 12.),
        spawn_mesh_at(path,  3.,  6., 10.),
        spawn_mesh_at(path,  6., -2., 14.),
    ];

    let mut window = Window::new(WINDOW_TITLE,
            WIDTH, HEIGHT, WindowOptions::default())
            .expect("Failed to create window!");

    let mut camera = Camera::default();
    camera.fov = 1.;
    let mut renderer = Renderer::new(WIDTH, HEIGHT, camera);

    let fps_alpha = 0.1;
    let mut fps: f64 = 0.;

    let  scale_vec = Vec3::new(0.02, 0.02, 0.02);

    while window.is_open() {
        let start_time = Instant::now();
        if window.is_key_pressed(minifb::Key::Escape, minifb::KeyRepeat::No) {
            break;
        }

        let delta = 0.1;
        if window.is_key_pressed(minifb::Key::D, minifb::KeyRepeat::Yes) {
            renderer.camera.position.x += delta;
        }
        if window.is_key_pressed(minifb::Key::A, minifb::KeyRepeat::Yes) {
            renderer.camera.position.x -= delta;
        }
        if window.is_key_pressed(minifb::Key::W, minifb::KeyRepeat::Yes) {
            renderer.camera.position.y += delta;
        }
        if window.is_key_pressed(minifb::Key::S, minifb::KeyRepeat::Yes) {
            renderer.camera.position.y -= delta;
        }
        if window.is_key_pressed(minifb::Key::Z, minifb::KeyRepeat::Yes) {
            renderer.camera.position.z += delta;
        }
        if window.is_key_pressed(minifb::Key::X, minifb::KeyRepeat::Yes) {
            renderer.camera.position.z -= delta;
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

fn spawn_mesh_at(path: &str, x: f32, y: f32, z: f32) -> Mesh {
    let obj_str = fs::read_to_string(path).unwrap();
    let mut mesh = parser::parse_polygon(&obj_str).unwrap();
    let start_loc = Vec3::new(x, y, z);
    mesh.translation.add(&start_loc);
    mesh
}

