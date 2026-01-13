#![allow(dead_code, unused)]
mod color;
mod vec3;

use std::io::{Write, stdout};

use color::{Color, write_color};

fn main() {
    let image_height = 256;
    let image_width = 256;

    println!("P3\n{image_width} {image_height}\n255");

    for j in 0..image_height {
        eprint!("\rscanlines remaining: {}", image_height - j);
        let _ = std::io::stderr().flush();
        for i in 0..image_width {
            let col = Color::new(
                (i as f64) / (image_width - 1) as f64,
                (j as f64) / (image_width - 1) as f64,
                0.0,
            );

            write_color(&mut stdout(), &col);
        }
    }
    eprintln!("\rDone.                        ");
}
