use crate::primitives::color::Color;
use crate::material::Material;
use crate::primitives::ray::Ray;
use crate::primitives::hitrecord::Hitrecord;
use crate::primitives::point3::random_unit_vector;

#[derive(Clone, Copy)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo : Coloralbedo : Color) {
        unimplemented!();
    }
}


impl Material for Lambertian {
    fn scatter(&self, _r_in :&Ray, rec : &mut Hitrecord, attenuation : &mut Color ,scattered: &mut Ray) -> bool {
        let scatter_direction = rec.normal + random_unit_vector();
        *scattered = Ray::ray(rec.p, scatter_direction);
        *attenuation = self.albedo;
        return true;
    }
}



pub fn lambertian() -> Lambertian {
    Lambertian { albedo }
}