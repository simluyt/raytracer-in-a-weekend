use crate::primitives::color::Color;
use crate::material::materialize::Materialize;
use crate::primitives::ray::Ray;
use crate::primitives::hitrecord::Hitrecord;
use crate::primitives::point3::random_unit_vector;

#[derive(Clone, Copy)]
pub struct Lambertian {
    albedo: Color,
}


impl Materialize for Lambertian {
    fn scatter(&self, _r_in :&Ray, rec : &mut Hitrecord, attenuation : &mut Color ,scattered: &mut Ray) -> bool {
        let scatter_direction = rec.normal + random_unit_vector();
        *scattered = Ray::ray(rec.p, scatter_direction);
        *attenuation = self.albedo;
        return true;
    }
}

pub fn lambertian(albedo : Color) -> Lambertian {
    Lambertian { albedo }
}