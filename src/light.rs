
use crate::model::Vec3;
use crate::model::Color;


pub enum Light {
    Sun { direction: Vec3, intensity: f32, color: Color },
    Point { location: Vec3, intensity: f32, color: Color },
    Spot { location: Vec3, direction: Vec3, angle: f32, intensity: f32, color: Color }
}
