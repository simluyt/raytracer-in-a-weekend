use std::rc::Rc;

use crate::primitives::hitrecord::Hitrecord;
use crate::primitives::hittable::Hittable;
use crate::primitives::point3::Point3;
use crate::primitives::ray::Ray;
use crate::primitives::vec3::dot;

use super::material::Material;

pub(crate) struct Sphere {
    center: Point3,
    radius: f64,
    mat_ptr: Rc<dyn Material>,
}

impl Sphere {
    pub fn sphere(center: Point3, radius: f64, mat_ptr: Rc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            mat_ptr,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut Hitrecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = dot(oc, r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root: f64 = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat_ptr = self.mat_ptr.clone();

        true
    }
}
