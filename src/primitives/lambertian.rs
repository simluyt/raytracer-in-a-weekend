use super::{
    color::Color, hitrecord::Hitrecord, material::Material, point3::random_unit_vector, ray::Ray,
};

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &Hitrecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let scatter_direction = hit_record.normal + random_unit_vector();
        if scatter_direction.nearly_zero() {
            *scattered = Ray::ray(hit_record.p, hit_record.normal);
        } else {
            *scattered = Ray::ray(hit_record.p, scatter_direction);
        }
        *attenuation = self.albedo;
        true
    }
}
