use crate::math::rand::random_float;
use crate::primitives::camera::Camera;
use primitives::color::{color, write_color, Color};
use primitives::hitrecord::Hitrecord;
use primitives::hittable::Hittable;
use primitives::hittables::Hittables;
use primitives::lambertian::Lambertian;
use primitives::metal::Metal;
use primitives::point3::{point3, random_in_hemisphere, Point3};
use primitives::ray::Ray;
use primitives::sphere::Sphere;
use primitives::vec3::{dot, unit};
use std::fs::File;
use std::io::{Error, Write};
use std::rc::Rc;

mod math;
mod primitives;
mod util;

fn hit_sphere(center: Point3, radius: f64, r: Ray) -> f64 {
    let oc = r.origin() - center;
    let a = r.direction().length_squared();
    let half_b = dot(oc, r.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        -half_b - discriminant.sqrt() / a
    }
}

fn ray_color(r: Ray, world: &mut Hittables, depth: i32) -> Color {
    let mut rec = Hitrecord::empty();

    if depth <= 0 {
        return color(0.0, 0.0, 0.0);
    }

    if world.hit(&r, 0.001, std::f64::INFINITY, &mut rec) {
        let mut scattered = Ray::new();
        let mut attenuation = color(0.0, 0.0, 0.0);
        //let target = rec.p + rec.normal + random_unit_vector();
        //let target = rec.p + random_in_hemisphere(rec.normal);
        //return 0.5* ((rec.normal + color(1.0,1.0,1.0)));
        //return 0.5 * ray_color(Ray::ray(rec.p, target - rec.p), world, depth - 1);
        if rec
            .mat_ptr
            .scatter(&r, &rec, &mut attenuation, &mut scattered)
        {
            return attenuation * ray_color(scattered, world, depth - 1);
        }
    }

    let unit_dir = unit(r.direction());
    let t = 0.5 * (unit_dir.y + 1.0);
    (1.0 - t) * color(1.0, 1.0, 1.0) + t * color(0.5, 0.7, 1.0)
}

fn main() -> Result<(), Error> {
    // Image
    let ratio = 16.0 / 9.0;

    let w = 400;
    let h = (w as f64 / ratio) as i32;

    let samples = 100;

    let max: i32 = 50;

    // World

    let world: &mut Hittables = &mut Hittables { items: vec![] };

    let material_ground = Rc::new(Lambertian::new(color(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(color(0.7, 0.3, 0.3)));
    let material_left = Rc::new(Metal::new(color(0.8, 0.8, 0.8)));
    let material_right = Rc::new(Metal::new(color(0.8, 0.6, 0.2)));

    world.add(Box::new(Sphere::sphere(
        point3(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Box::new(Sphere::sphere(
        point3(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Box::new(Sphere::sphere(
        point3(1.0, 0.0, -1.0),
        0.5,
        material_right.clone(),
    )));
    world.add(Box::new(Sphere::sphere(
        point3(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    )));

    // Camera

    let camera = Camera::camera();

    // Render

    let path = "lines.ppm";
    let mut output = File::create(path)?;

    // Use match to handle error gracefully
    match write!(output, "P3\n{} {}\n255\n", w, h) {
        Ok(_) => {}
        Err(e) => eprintln!("Failed to write to file: {}", e),
    }

    for j in (0..(h)).rev() {
        //Basic loading
        print!("\rScanlines remaining : {} of {}", j, h);
        for i in 0..w {
            let mut pixel_color = color(0.0, 0.0, 0.0);
            for _s in 0..samples {
                let u = (i as f64 + random_float()) / (w - 1) as f64;
                let v = (j as f64 + random_float()) / (h - 1) as f64;
                let r: Ray = camera.get_ray(u, v);
                pixel_color += ray_color(r, world, max);
            }
            write_color(&mut output, pixel_color, samples);
        }
    }

    Ok(())
}
