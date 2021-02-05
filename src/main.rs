use std::fs::File;
use std::io::{Write, Error};
use primitives::vec3::{unit, dot};
use primitives::color::{Color, color, write_color};
use primitives::ray::{Ray};
use primitives::point3::{Point3, point3};
use primitives::hittable::Hittable;
use primitives::hittables::Hittables;
use primitives::hitrecord::Hitrecord;
use primitives::sphere::Sphere;
use crate::primitives::camera::Camera;
use crate::math::rand::random_float;
use crate::primitives::point3::{random_in_unit_sphere, random_unit_vector, random_in_hemisphere};
use material::lambertian::{lambertian};
use material::metal::{metal};
use crate::primitives::vec3::Vec3;

mod primitives;
mod material;
mod util;
mod math;

fn hit_sphere( center: Point3, radius : f64, r: Ray) -> f64 {
    let oc = r.origin() - center;
    let a = r.direction().length_squared();
    let half_b = dot(oc, r.direction());
    let c = oc.length_squared() -radius * radius;
    let discriminant = half_b* half_b - a*c;
        if discriminant < 0.0 {
            -1.0
        } else {
            -half_b - discriminant.sqrt() / a
        }
}

fn ray_color(r : Ray, world: &mut Hittables, depth: i32) -> Color {
    let mut rec = Hitrecord::new_empty();

    if depth <= 0 {
        return color(0.0,0.0,0.0);
    }

    if world.hit(&r, 0.001, std::f64::INFINITY, &mut rec) {
        let mut scattered: Ray = Ray::ray(Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0
        }, Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0
        });
        let mut attenuation: Color = color(0.0, 0.0, 0.0);
        if rec.material.scatter(&r, &mut rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(scattered,world,depth-1);
        }
        //let target = rec.p + rec.normal + random_unit_vector();
        //let target = rec.p + random_in_hemisphere(rec.normal);
        //return 0.5* ((rec.normal + color(1.0,1.0,1.0)));
        //return 0.5* ray_color(Ray::ray(rec.p,target-rec.p), world,depth-1);
        return color(0.0,0.0,0.0);
    }

    let unit_dir = unit(r.direction());
    let t = 0.5*(unit_dir.y + 1.0);
    (1.0-t)* color(1.0,1.0,1.0) + t * color(0.5, 0.7, 1.0)
}



fn main() -> Result<(), Error> {

    // Image
    let ratio = 16.0/9.0;

    let w = 400;
    let h = (w as f64 / ratio) as i32;

    let samples = 100;

    let max : i32 = 5;

    // World

    let world : &mut Hittables = &mut Hittables {
        items: vec![]
    };

    let material_ground = lambertian(color(0.8, 0.8, 0.0));
    let material_center = lambertian(color(0.7, 0.3, 0.3));
    let material_left   = metal(color(0.8, 0.8, 0.8));
    let material_right  = metal(color(0.8, 0.6, 0.2));

    world.add(Sphere::sphere(point3(0.0,0.0,-1.0), 0.5, Box::new(material_ground)));
    world.add(Sphere::sphere(point3(0.0,-100.5,-1.0), 100.0, Box::new(material_center)));


    // Camera

    let camera = Camera::camera();

    // Render

    let path = "lines.ppm";
    let mut output = File::create(path)?;

    write!(output, "P3\n{} {}\n255\n", w ,h);

    for j in (0..(h)).rev() {
        //Basic loading
        print!("\rScanlines remaining : {} of {}", j,h);
        for i in 0..w {

            let mut pixel_color = color(0.0, 0.0, 0.0);
            for _s in 0..samples {
                let u = (i as f64 + random_float()) / (w-1) as f64;
                let v = (j as f64 + random_float()) / (h-1) as f64;
                let r : Ray = camera.get_ray(u, v);
                pixel_color += ray_color(r, world, max);
            }
            write_color(&mut output, pixel_color, samples);
        }
    }



    Ok(())



}


