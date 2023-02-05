use crate::primitives::vec3::{Vec3, random_vector_range, unit};

use super::vec3::dot;

pub type Point3 = Vec3;

pub fn point3( x : f64, y : f64, z: f64) -> Point3 {
    Vec3 { x , y , z }
}

pub fn random_in_unit_sphere() -> Point3 {
     loop {
        let p : Point3 = random_vector_range(-1,1);
        if p.length_squared() >= 1.0 {
            continue;
        };

        return p;
    }
}

pub fn random_unit_vector() -> Vec3{
    return unit(random_in_unit_sphere());
}

pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if dot(in_unit_sphere, normal) > 0.0 {
        return in_unit_sphere;
    } else {
        return -in_unit_sphere;
    }
}