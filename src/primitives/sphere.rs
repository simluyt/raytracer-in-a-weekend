use crate::primitives::hittable::Hittable;
use crate::primitives::hitrecord::Hitrecord;
use crate::primitives::ray::Ray;
use crate::primitives::vec3::dot;
use crate::primitives::point3::Point3;
use crate::material::Material;
use std::rc::Rc;

pub struct Sphere {
    center: Point3,
    radius : f64,
    material : Rc<Box<dyn Material>>,
}

impl Sphere {
    pub fn sphere(center : Point3, radius: f64, material: Rc<Box<dyn Material>>) -> Box<Sphere> {
        Box::new(Sphere{ center, radius, material})
    }
}

impl Hittable for Sphere {
    fn hit(&self ,r :&Ray,t_min : f64, t_max : f64, rec : &mut Hitrecord ) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = dot(oc, r.direction());
        let c = oc.length_squared() - self.radius*self.radius;

        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 { return false; }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root : f64 = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root { return false; }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p -self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.material = self.material.clone();

        true
    }
}