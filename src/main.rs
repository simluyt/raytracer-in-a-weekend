use std::fs::File;
use std::io::{Write, Error, stdout};
use crate::types::color::{Color, write_color};

mod types;


fn main() -> Result<(), Error> {
    let path = "lines.ppm";

    let mut output = File::create(path)?;
    let w = 256;
    let h = 256;

    write!(output, "P3\n{} {}\n255\n", w ,h);

    for j in (0..(h)).rev() {
        for i in 0..w {

            let r = i as f64 / (w-1) as f64;
            let g = j as f64 / (h-1) as f64;
            let b = 0.25;



            let ir = 255.999 * r;
            let ig = 255.999 * g;
            let ib = 255.999 * b;


            let color = Color { x: r, y: g, z: b };
            write_color(&mut output, color);
        }
    }

    let vector = types::vec3::Vec3 { x: 1.0, y: 2.0, z: 3.0};
    println!("{:?} and {}", vector.length_squared(), vector);



    Ok(())



}


