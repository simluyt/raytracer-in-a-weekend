use std::fs::File;
use std::io::{Write, Error};
use crate::types::vec3::{Vec3, unit, dot, vector, color, point3};
use crate::types::color::Color;
use crate::types::ray::{Ray};
use crate::types::point3::Point3;

mod types;

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

fn ray_color(r : Ray) -> Color {
    let t = hit_sphere(point3(0.0,0.0,-1.0), 0.5, r);
    if t > 0.0 {
        let N = unit(r.at(t) - vector(0.0, 0.0, -1.0));
        return 0.5* color( N.x + 1.0, N.y + 1.0, N.z + 1.0 );
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


    // Camera

    let vp_h = 2.0;
    let vp_w = ratio * vp_h;
    let focal_length = 1.0;

    let origin = point3(0.0, 0.0,  0.0);
    let horizontal = vector (vp_w, 0.0, 0.0);
    let vertical = vector ( 0.0, vp_h,0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - vector(0.0, 0.0,focal_length);

    // Render

    let path = "lines.ppm";
    let mut output = File::create(path)?;

    write!(output, "P3\n{} {}\n255\n", w ,h);

    for j in (0..(h)).rev() {
        print!("\rScanlines remaining : {} of {}", j,h);
        for i in 0..w {

            let u = i as f64 / (w-1) as f64;
            let v = j as f64 / (h-1) as f64;

            let r = Ray::ray(origin, lower_left_corner  + u*horizontal + v*vertical - origin);
            let color = ray_color(r);
            types::color::write_color(&mut output, color);
        }
    }



    Ok(())



}


