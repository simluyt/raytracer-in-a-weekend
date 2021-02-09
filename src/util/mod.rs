use rand::prelude::*;

pub fn degrees_to_radians(degrees : f64) -> f64 {
    degrees * (std::f64::consts::PI / 180.0)
}

pub fn clamp(x : f64, min : f64, max : f64) -> f64 {
    if x < min { return min; }
    if x > max { return max; }
    return x;
}

pub fn random_float() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub fn random_float_range(min: i32, max: i32) -> f64 {
    (min as f64) + ((max-min) as f64)*random_float()
}