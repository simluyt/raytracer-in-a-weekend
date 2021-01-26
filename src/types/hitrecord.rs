use crate::types::point3::Point3;
use crate::types::vec3::{Vec3, dot, point3, vector};
use crate::types::ray::Ray;

#[derive(Clone, Copy)]
pub struct Hitrecord {
    pub p : Point3,
    pub normal : Vec3,
    pub t: f64,
    pub front: bool

}

impl Hitrecord {

    pub fn rec(p : Point3, normal: Vec3, t : f64, front: bool) -> Hitrecord {
        Hitrecord{ p, normal, t ,front}
    }

    pub fn new_empty() -> Hitrecord {
        let p = point3(0.0,0.0,0.0);
        let normal = vector(0.0,0.0,0.0);
        let t = 0.0;
        let front = false;
        Hitrecord::rec(p,normal,t,front)
    }

    pub fn set_face_normal(&mut self, r : &Ray , outward_normal:  Vec3) {
        self.front = dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front { outward_normal } else { -outward_normal };
    }
}