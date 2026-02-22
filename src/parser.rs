
use crate::Vec3;
use crate::model::Face;
use crate::model::Polygon;


pub fn parse_vertices(source: &str) -> Vec<Vec3> {
    let lines = source.lines().collect::<Vec<&str>>();
    lines.iter()
        .filter_map(|l| l.strip_prefix("v "))
        .map(|p| {
            let s = p.split(" ").collect::<Vec<&str>>();
            let x = s[0].parse::<f32>().expect(&format!("Invalid X for vertex: {}", p));
            let y = s[1].parse::<f32>().expect(&format!("Invalid Y for vertex: {}", p));
            let z = s[2].parse::<f32>().expect(&format!("Invalid Z for vertex: {}", p));
            Vec3::new(x, y, z)
        })
        .collect()
}

pub fn parse_polygon(source: &str) -> Result<Polygon, String> {
    let lines = source.lines().collect::<Vec<&str>>();
    let vertices: Vec<Vec3> = lines.iter()
        .filter_map(|l| l.strip_prefix("v "))
        .map(|p| {
            let s = p.split(" ").collect::<Vec<&str>>();
            let x = s[0].parse::<f32>().expect(&format!("Invalid X for vertex: {}", p));
            let y = s[1].parse::<f32>().expect(&format!("Invalid Y for vertex: {}", p));
            let z = s[2].parse::<f32>().expect(&format!("Invalid Z for vertex: {}", p));
            Vec3::new(x, y, z)
        })
        .collect();

    let faces: Vec<Face> = lines.iter()
        .filter_map(|l| l.strip_prefix("f "))
        .map(|f| {
            let s = f.split(" ").collect::<Vec<&str>>();
            let mut v = Vec::new();
            for p in s {
                let idx = if p.contains(&"/") {
                    p.split("/").collect::<Vec<&str>>()[0]
                }
                else {
                    p
                };
                let idx = idx.parse::<u32>().expect(&format!("Invalid index for face: {}", f)) - 1;
                if idx >= vertices.len() as u32 {
                    panic!("Invalid face for polygon: {}", f);
                }
                v.push(idx);
            }
            Face::with_points(v)
            // v
        })
        .collect();

    Ok(Polygon::with_points_and_faces(vertices, faces))
    // todo!()
}
