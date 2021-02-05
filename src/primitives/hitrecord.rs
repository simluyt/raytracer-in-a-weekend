use crate::primitives::point3::{Point3, point3};
use crate::primitives::vec3::{Vec3, vector, dot};
use crate::primitives::ray::Ray;
use crate::material::materialize::Materialize;

#[derive(Clone)]
pub struct Hitrecord {
    pub p : Point3,
    pub normal : Vec3,
    pub material : Box<dyn Materialize>,
    pub t: f64,
    pub front: bool

}

impl Hitrecord {

    pub fn rec(p : Point3, normal: Vec3, material: Box<dyn Materialize>,t : f64, front: bool) -> Hitrecord {
        Hitrecord{ p, normal,material, t ,front}
    }

    pub fn new_empty() -> Hitrecord {
        let p = point3(0.0,0.0,0.0);
        let normal = vector(0.0,0.0,0.0);
        let material : Box<dyn Materialize> = Box::new(());
        let t = 0.0;
        let front = false;
        Hitrecord::rec(p,normal,material,t,front)
    }

    pub fn set_face_normal(&mut self, r : &Ray , outward_normal:  Vec3) {
        self.front = dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front { outward_normal } else { -outward_normal };
    }
}