use super::{
    color::Color,
    hitrecord::Hitrecord,
    material::Material,
    point3::random_unit_vector,
    ray::Ray,
    vec3::{dot, reflect, unit},
};

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Metal {
        Metal { albedo }
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
        *scattered = Ray::ray(hit_record.p, reflected);
        *attenuation = self.albedo;
        dot(scattered.direction(), hit_record.normal) > 0.0
    }
}
