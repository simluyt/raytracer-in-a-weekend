use crate::types::ray::Ray;
use crate::types::hitrecord::Hitrecord;

pub trait Hittable {

    /// Checks if the implementing object is hit by a ray.
    #[must_use]
    fn hit(&self,_r :&Ray,_t_min : f64, _t_max : f64, _rec : &mut Hitrecord ) -> bool { true }
}