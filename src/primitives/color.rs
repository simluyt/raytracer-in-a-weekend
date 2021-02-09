use std::io::{Write};
use std::fs::File;
use crate::util::clamp;
use crate::primitives::vec3::Vec3;

pub type Color = Vec3;



pub fn write_color(out: &mut File, pixel : Color, samples : i32) {
    let mut r = pixel.x;
    let mut g = pixel.y;
    let mut b = pixel.z;

    let scale = 1.0 / samples as f64;
    r *= scale;
    g *= scale;
    b *= scale;

    let r = format!("{} {} {}\n",
                    256.0 * clamp(r.sqrt(), 0.0, 0.999),
                    256.0 * clamp(g.sqrt(), 0.0, 0.999),
                    256.0 * clamp(b.sqrt(), 0.0, 0.999));

    out.write(r.as_bytes());
}

pub fn color( x : f64, y : f64, z: f64) -> Color {
    Vec3 { x , y , z }
}