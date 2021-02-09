pub mod lambertian;
pub mod metal;

use crate::primitives::ray::Ray;
use crate::primitives::hitrecord::Hitrecord;
use crate::primitives::color::Color;

pub trait Material {
    #[must_use]
    fn scatter(&self,_r_in :&Ray, _rec : &mut Hitrecord, attenuation : &mut Color, scattered : &mut Ray ) -> bool { false }
}