use crate::primitives::hitrecord::Hitrecord;
use crate::primitives::ray::Ray;

pub trait Hittable {
    /// Checks if the implementing object is hit by a ray.
    #[must_use]
    fn hit(&self, _r: &Ray, _t_min: f64, _t_max: f64, _rec: &mut Hitrecord) -> bool {
        true
    }
}
