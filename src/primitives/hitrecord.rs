use std::rc::Rc;

use crate::primitives::point3::{point3, Point3};
use crate::primitives::ray::Ray;
use crate::primitives::vec3::{dot, vector, Vec3};

use super::color::color;
use super::material::Material;
use crate::primitives::lambertian::Lambertian;

#[derive(Clone)]
pub struct Hitrecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat_ptr: Rc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl Hitrecord {
    pub fn new(
        p: Point3,
        normal: Vec3,
        mat_ptr: Rc<dyn Material>,
        t: f64,
        front_face: bool,
    ) -> Hitrecord {
        Hitrecord {
            p,
            normal,
            mat_ptr,
            t,
            front_face,
        }
    }

    pub fn empty() -> Hitrecord {
        let p = point3(0.0, 0.0, 0.0);
        let normal = vector(0.0, 0.0, 0.0);
        let mat_ptr = Rc::new(Lambertian::new(color(0.0, 0.0, 0.0)));
        let t = 0.0;
        let front_face = false;
        Hitrecord::new(p, normal, mat_ptr, t, front_face)
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = dot(ray.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
