
// use macroquad::color::LIME;
// use macroquad::color::YELLOW;
// use macroquad::color::Color;
// use macroquad::prelude::glam;
// use macroquad::shapes::draw_line;
// use macroquad::shapes::draw_circle;

// use crate::project;
// use crate::model::Vec2;
// use crate::model::Mesh;
// use crate::model::Triangle;


// pub fn draw_vertices(p: &Mesh) {
//     for v in &p.transformed_points() {
//         let proj = project::val_project(&v);
//         if let Some(p) = proj {
//             let sp = project::to_screen(&p);
//             draw_circle(sp.x, sp.y, 1.5, YELLOW);
//         }
//     }
// }

// pub fn draw_wireframe(p: &Mesh) {
//     let points = p.transformed_points();
//     if points.len() > 1 {
//         for i in 0..points.len() - 1 {
//             if let Some((v1, v2)) = project::val_project_segment(
//                     &points[i], &points[i + 1]) {
//                 draw_line(v1.x, v1.y, v2.x, v2.y, 2.0, LIME);
//             }
//         }
//         if let Some((v1, v2)) = project::val_project_segment(
//                     &points[points.len() - 1], &points[0]) {
//             draw_line(v1.x, v1.y, v2.x, v2.y, 2.0, LIME);
//         }
//     }
//     else {
//         println!("Invalid Polygon");
//     }
// }

// pub fn draw_defined_wireframe(polygon: &Mesh) {
//     let verts = &polygon.transformed_points();
//     for face in &polygon.faces {
//         if face.points.len() > 1 {
//             for i in 0..face.points.len() - 1 {
//                 if let Some((v1, v2)) = project::val_project_segment(
//                         &verts[face.points[i] as usize],
//                         &verts[face.points[i + 1] as usize]) {
//                     draw_line(v1.x, v1.y, v2.x, v2.y, 1.0, LIME);
//                 }
//             }
//             if let Some((v1, v2)) = project::val_project_segment(
//                         &verts[face.points[face.points.len() - 1] as usize],
//                         &verts[face.points[0] as usize]) {
//                 draw_line(v1.x, v1.y, v2.x, v2.y, 1.0, LIME);
//             }
//         }
//     }
// }

// pub fn draw_defined_faces(polygon: &Mesh) {
//     let verts = &polygon.transformed_points();
//     let mut draw_list: Vec<(Vec2, Vec2, Vec2, Color, f32)> = Vec::new();

//     for face in &polygon.faces {
//         if face.points.len() >= 3 {
//             let r_i = face.points[0];
//             let root = &verts[r_i as usize];
//             if let Some(v1) = project::val_project(&root) {
//                 let v1 = project::to_screen(&v1);
//                 for i in 1..face.points.len() - 1 {
//                     let p2 = &verts[face.points[i] as usize];
//                     let p3 = &verts[face.points[i + 1] as usize];
//                     if let Some((v2, v3)) = project::val_project_segment(
//                             p2, p3) {
//                         let triag = Triangle::new(
//                                 root.clone(), p2.clone(), p3.clone());
//                         let normal = triag.normal().unit();
//                         if normal.z < 0. {
//                             let r = ((normal.x + 1.) * 127.) as u8;
//                             let g = ((normal.y + 1.) * 127.) as u8;
//                             let b = ((normal.z + 1.) * 127.) as u8;
//                             // let c = Color::from_rgba(r, g, b, 192);
//                             // let c = Color::from_rgba(r, g, b, 128);
//                             let c = Color::from_rgba(r, g, b, 255);
//                             let z_ord = (root.z + p2.z + p3.z) / 3.;

//                             draw_list.push((v1.clone(), v2, v3, c, z_ord));
//                         }
//                     }
//                 }
//             }
//         }
//         else {
//             println!("Invalid Polygon");
//         }
//     }

//     draw_list.sort_by(|a, b| b.4.total_cmp(&a.4));
//     draw_list.iter().for_each(|(v1, v2, v3, c, _z_ord)|
//             draw_triangle_colored(&v1, &v2, &v3, *c));
// }

// pub fn draw_faces(p: &Mesh) {
//     if p.points.len() >= 3 {
//         if let Some(v1) = project::val_project(&p.points[0]) {
//             let v1 = project::to_screen(&v1);
//             for i in 1..p.points.len() - 1 {
//                 if let Some((v2, v3)) = project::val_project_segment(
//                         &p.points[i], &p.points[i + 1]) {
//                     draw_triangle(&v1, &v2, &v3);
//                 }
//             }
//         }
//     }
//     else {
//         println!("Invalid Polygon");
//     }
// }

// fn draw_triangle(v1: &Vec2, v2: &Vec2, v3: &Vec2) {
//     let gv1 = glam::vec2(v1.x, v1.y);
//     let gv2 = glam::vec2(v2.x, v2.y);
//     let gv3 = glam::vec2(v3.x, v3.y);
//     let color = Color::from_rgba(64, 128, 192, 192);
//     macroquad::prelude::draw_triangle(gv1, gv2, gv3, color);
// }

// fn draw_triangle_colored(v1: &Vec2, v2: &Vec2, v3: &Vec2, c: Color) {
//     let gv1 = glam::vec2(v1.x, v1.y);
//     let gv2 = glam::vec2(v2.x, v2.y);
//     let gv3 = glam::vec2(v3.x, v3.y);
//     macroquad::prelude::draw_triangle(gv1, gv2, gv3, c);
// }



