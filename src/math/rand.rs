use rand::prelude::*;

pub fn random_float() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub fn random_float_range(min: i32, max: i32) -> f64 {
    (min as f64) + ((max-min) as f64)*random_float()
}