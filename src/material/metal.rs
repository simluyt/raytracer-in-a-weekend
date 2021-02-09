use crate::primitives::color::Color;
use crate::primitives::ray::Ray;
use crate::primitives::hitrecord::Hitrecord;
use crate::primitives::vec3::{reflect, unit};
use crate::material::Material;

pub struct Metal {
    albedo: Color,
}


impl Material for Metal {
    fn scatter(&self, _r_in :&Ray, rec : &mut Hitrecord, attenuation : &mut Color ,scattered: &mut Ray) -> bool {
        let reflected = reflect(&unit(_r_in.direction()), &rec.normal);
        *scattered = Ray::ray(rec.p, reflected);
        *attenuation = self.albedo;
        return true;
    }
}



pub fn metal(albedo : Color) -> Metal{
    Metal { albedo }
}