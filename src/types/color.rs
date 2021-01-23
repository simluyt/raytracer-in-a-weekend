use std::io::{Write, Stdout};
use crate::types::vec3::Vec3;
use std::fs::File;

pub type Color = Vec3;



pub fn write_color(out: &mut File, pixel : Color) {
    let r = format!("{} {} {}\n", 255.999 * pixel.x, 255.999 * pixel.y, 255.999 * pixel.z);
    out.write(r.as_bytes());
}