use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};

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

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            writeln!(output, "{} {} {}", ir, ig, ib);
        }
    }

    Ok(())



}


