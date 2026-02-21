
use macroquad::color::GREEN;
use macroquad::color::LIME;
use macroquad::color::RED;
use macroquad::color::BLUE;
use macroquad::color::BLACK;
use macroquad::color::WHITE;
use macroquad::input::KeyCode;
use macroquad::input::is_key_pressed;
use macroquad::text::draw_text;
use macroquad::shapes::draw_line;
use macroquad::shapes::draw_circle;
use macroquad::time::draw_fps;
use macroquad::window::Conf;
use macroquad::window::next_frame;
use macroquad::window::clear_background;


#[macroquad::main(window_conf)]
async fn main() {
    let init_verts = vec![
        Vec3::new(-0.5, -0.5, 0.75),
        Vec3::new(-0.5,  0.5, 0.75),
        Vec3::new( 0.5,  0.5, 0.75),
        Vec3::new( 0.5, -0.5, 0.75),
    ];
    let init_poly = Polygon::with_ponts(init_verts);
    // let mut verts = vec![ ];
    let mut polys = vec![];
    for it in 0..15 {
        let off = it as f32 * 1.;
        let v_off = Vec3::new(0., 0., off);
        let mut p = init_poly.clone();
        p.translate(&v_off);
        polys.push(p);

        // for v in &init_verts {
        //     verts.push(Vec3::new(v.x, v.y, v.z + off));
        // }
    }
    loop {
        if is_key_pressed(KeyCode::Q) {
            break;
        }

        clear_background(BLACK);
        draw_fps();

        let m_v = Vec3::new(0., 0., -0.02);
        polys.iter_mut().for_each(|p| p.translate(&m_v));

        for p in &polys {
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

        // for v in &verts {
        //     let proj = val_project(&v);
        //     if let Some(p) = proj {
        //         let sp = to_screen(&p);
        //         // println!("{:#?}", &sp);
        //         // draw_circle(sp.x, sp.y, 2., GREEN);
        //         draw_circle(sp.x, sp.y, 2., LIME);
        //     }
        // }

        next_frame().await;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Simple 3D".to_string(),
        window_width: 800,
        window_height: 600,
        fullscreen: false,
        ..Default::default()
    }
}

fn val_project_segment(v1: &Vec3, v2: &Vec3) -> Option<(Vec2, Vec2)> {
    let opt1 = &val_project(v1);
    let opt2 = &val_project(v2);
    if opt1.is_some() && opt2.is_some() {
        Some((to_screen(opt1.as_ref().unwrap()), to_screen(opt2.as_ref().unwrap())))
    }
    else {
        None
    }
}

fn val_project(v: &Vec3) -> Option<Vec2> {
        // avoid div by zero
        // cull behind camera
        // cull out of view
    if v.z < 0.00001
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
    let h = 600.;
    let x = (v.x + 1.) / 2.;
    let y = (v.y + 1.) / 2.;

    Vec2::new(x * w, y * h)
}

fn project(v: &Vec3) -> Vec2 {
    let px = v.x / v.z;
    let py = v.y / v.z;
    Vec2::new(px, py)
}

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
}
impl Polygon {
    fn new() -> Self {
        Self { points: Vec::new() }
    }
    fn with_ponts(points: Vec<Vec3>) -> Self {
        Self { points }
    }
    fn translate(&mut self, t: &Vec3) {
        self.points.iter_mut().for_each(|v| v.add(t));
    }
}
