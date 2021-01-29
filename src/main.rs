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

mod primitives;
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

fn ray_color(r : Ray, world: &mut Hittables) -> Color {
    let mut rec = Hitrecord::new_empty();
    if world.hit(&r, 0.0, std::f64::INFINITY, &mut rec) {
        return 0.5* (rec.normal + color(1.0,1.0,1.0))
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

    // World

    let world : &mut Hittables = &mut Hittables {
        items: vec![]
    };
    world.add(Box::new(Sphere::sphere(point3(0.0,0.0,-1.0), 0.5)));
    //world.add(Box::new(Sphere::sphere(point3(0.0,-100.5,-1.0), 100.0)));


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
                pixel_color += ray_color(r, world);
            }
            write_color(&mut output, pixel_color, samples);
        }
    }



    Ok(())



}


