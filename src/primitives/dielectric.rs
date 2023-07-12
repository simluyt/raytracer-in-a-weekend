use super::{
    color::{color as c, Color},
    hitrecord::Hitrecord,
    material::Material,
    point3::random_unit_vector,
    ray::Ray,
    vec3::{dot, reflect, refract, unit, Vec3},
};

pub struct Dielectric {
    index_of_refraction: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Dielectric {
        Dielectric {
            index_of_refraction,
        }
    }
}
impl Material for Dielectric {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &Hitrecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = c(1.0, 1.0, 1.0);
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let unit_direction = unit(ray_in.direction());
        let cos_theta = (dot(-unit_direction, hit_record.normal)).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract {
            reflect(unit_direction, hit_record.normal)
        } else {
            refract(unit_direction, hit_record.normal, refraction_ratio)
        };

        *scattered = Ray::ray(hit_record.p, direction);
        true
    }
}
