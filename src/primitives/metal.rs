use super::{
    color::Color,
    hitrecord::Hitrecord,
    material::Material,
    point3::random_in_unit_sphere,
    ray::Ray,
    vec3::{dot, reflect, unit},
};

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &Hitrecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(unit(ray_in.direction()), hit_record.normal);
        *scattered = Ray::ray(
            hit_record.p,
            reflected + (self.fuzz * random_in_unit_sphere()),
        );
        *attenuation = self.albedo;
        dot(scattered.direction(), hit_record.normal) > 0.0
    }
}
