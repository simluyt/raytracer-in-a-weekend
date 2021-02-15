use crate::primitives::color::{Color, color};

pub struct Material {
    pub color: Color,
    pub diffuse: f64,
    pub specular: f64,
    pub specular_exponent: f64,
    pub reflectiveness: f64,
}

impl Material {
    pub fn new() -> Material {
        Material {
            color: color(0.0,0.0,0.0),
            diffuse: 0.0,
            specular: 0.0,
            specular_exponent: 0.0,
            reflectiveness: 0.0,
        }
    }
}
