use super::color::Color;
use super::hitrecord::Hitrecord;
use super::ray::Ray;

//a Trait that defines the material properties of a surface
pub trait Material {
    // Scatter a ray and return the scattered ray and attenuation
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &Hitrecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}
