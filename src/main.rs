use std::fs::File;
use std::io::{Write, Error};
use crate::types::vec3::{Vec3, unit, dot};
use crate::types::color::Color;
use crate::types::ray::Ray;
use crate::types::point3::Point3;

mod types;

fn hit_sphere( center: Point3, radius : f64, r: Ray) -> bool {
    let oc = r.origin() - center;
    let a = dot(r.direction(), r.direction());
    let b = 2.0 * dot(oc, r.direction());
    let c = dot(oc, oc) - radius*radius;
    let discriminant = b*b - 4.0*a*c;
        discriminant > 0.0
}

fn ray_color(ray : Ray) -> Color {
    if hit_sphere(Point3 { x : 0.0,y : 0.0,z : -1.0 }, 0.5, ray) {
        return Color { x: 1.0, y: 1.0, z: 1.0 }
    }

    let unit_dir = unit(ray.direction());
    let t = 0.5*(unit_dir.y + 1.0);
    (1.0-t)* Color{x : 1.0, y : 1.0, z : 1.0} + t * Color{x : 0.5, y : 0.7, z : 1.0}
}



fn main() -> Result<(), Error> {

    // Image
    let ratio = 16.0/9.0;

    let w = 400;
    let h = (w as f64 / ratio) as i32;


    // Camera

    let vp_h = 2.0;
    let vp_w = ratio * vp_h;
    let focal_length = 1.0;

    let origin = types::point3::Point3 { x: 0.0, y: 0.0, z: 0.0};
    let horizontal = types::vec3::Vec3 { x: vp_w, y: 0.0, z: 0.0};
    let vertical = types::vec3::Vec3 {x: 0.0, y : vp_h,z : 0.0};
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3{x : 0.0, y: 0.0, z: focal_length};

    // Render

    let path = "lines.ppm";
    let mut output = File::create(path)?;

    write!(output, "P3\n{} {}\n255\n", w ,h);

    for j in (0..(h)).rev() {
        print!("\rScanlines remaining : {} of {}", j,h);
        for i in 0..w {

            let u = i as f64 / (w-1) as f64;
            let v = j as f64 / (h-1) as f64;

            let ray = Ray { orig : origin, dir : lower_left_corner  + u*horizontal + v*vertical - origin};
            let color = ray_color(ray);
            types::color::write_color(&mut output, color);
        }
    }



    Ok(())



}


