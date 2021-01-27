use crate::primitives::vec3::Vec3;

pub type Point3 = Vec3;




pub fn point3( x : f64, y : f64, z: f64) -> Point3 {
    Vec3 { x , y , z }
}
