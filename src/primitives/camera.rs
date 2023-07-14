use crate::primitives::point3::{point3, Point3};
use crate::primitives::ray::Ray;
use crate::primitives::vec3::{vector, Vec3};

use super::vec3::{cross, unit};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn camera(
        loofrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
    ) -> Camera {
        let theta = vfov.to_radians();
        let h = (std::f64::consts::PI / 3.0).tan() / 2.0;
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let w = unit(loofrom - lookat);
        let u = unit(cross(vup, w));
        let v = cross(w, u);

        let origin = loofrom;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;
        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::ray(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
