use crate::math::rand::random_float;
use crate::primitives::camera::Camera;
use math::rand::random_float_range;
use primitives::color::{color, random_color, random_color_range, write_color, Color};
use primitives::dielectric::Dielectric;
use primitives::hitrecord::Hitrecord;
use primitives::hittable::Hittable;
use primitives::hittables::Hittables;
use primitives::lambertian::Lambertian;
use primitives::material::Material;
use primitives::metal::Metal;
use primitives::point3::{point3, Point3};
use primitives::ray::Ray;
use primitives::sphere::Sphere;
use primitives::vec3::{dot, unit, vector};
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

fn last_scene() -> Hittables {
    let mut world = Hittables { items: vec![] };

    let material_ground = Rc::new(Lambertian::new(color(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(color(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_right = Rc::new(Metal::new(color(0.8, 0.6, 0.2), 0.0));

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
    world.add(Box::new(Sphere::sphere(
        point3(-1.0, 0.0, -1.0),
        -0.4,
        material_left.clone(),
    )));

    world
}

fn random_scene() -> Hittables {
    let mut world = Hittables { items: vec![] };

    let ground_material = Rc::new(Lambertian::new(color(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::sphere(
        point3(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_float();
            let center = point3(
                a as f64 + 0.9 * random_float(),
                0.2,
                b as f64 + 0.9 * random_float(),
            );

            if (center - point3(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<dyn Material>;
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = random_color() * random_color();
                    sphere_material = Rc::new(Lambertian::new(albedo));
                    world.add(Box::new(Sphere::sphere(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = random_color_range(0.5, 1.0);
                    let fuzz = random_float_range(0.0, 0.5);
                    sphere_material = Rc::new(Metal::new(albedo, fuzz));
                    world.add(Box::new(Sphere::sphere(center, 0.2, sphere_material)));
                } else {
                    // glass
                    sphere_material = Rc::new(Dielectric::new(1.5));
                    world.add(Box::new(Sphere::sphere(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    let material2 = Rc::new(Lambertian::new(color(0.4, 0.2, 0.1)));
    let material3 = Rc::new(Metal::new(color(0.7, 0.6, 0.5), 0.0));

    world.add(Box::new(Sphere::sphere(
        point3(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    world.add(Box::new(Sphere::sphere(
        point3(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    world.add(Box::new(Sphere::sphere(
        point3(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}

fn main() -> Result<(), Error> {
    // Image
    let ratio = 3.0 / 2.0;

    let w = 600;
    let h = (w as f64 / ratio) as i32;

    let samples = 100;

    let max: i32 = 50;

    // World

    let mut world = random_scene();

    // Camera

    let lookfrom = point3(13.0, 2.0, 3.0);
    let lookat = point3(0.0, 0.0, 0.0);
    let vup = vector(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::camera(lookfrom, lookat, vup, 20.0, ratio, aperture, dist_to_focus);

    // Render

    let path = "lines2.ppm";
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
                pixel_color += ray_color(r, &mut world, max);
            }
            write_color(&mut output, pixel_color, samples);
        }
    }

    Ok(())
}
