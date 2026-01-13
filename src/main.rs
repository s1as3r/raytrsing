#![allow(dead_code, unused)]
mod color;
mod ray;
mod vec3;

use std::io::{Write, stdout};

use color::{Color, write_color};
use ray::Ray;
use vec3::Point3;

use crate::vec3::{Vec3, unit_vector};
use std::ops;

fn lerp<T>(start: T, end: T, t: f64) -> T
where
    T: ops::Mul<f64, Output = T> + ops::Add<Output = T>,
{
    start * (1.0 - t) + end * t
}

fn ray_color(r: &Ray) -> Color {
    let unit_direction = unit_vector(r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    lerp(Color::new(1.0, 1.0, 1.0), Color::new(0.5, 0.7, 1.0), a)
}

fn main() {
    let aspect_ratio = 16.0 / 10.0;

    let image_width = 400;
    let image_height = {
        let h = (image_width as f64 / aspect_ratio) as i32;
        if h < 1 { 1 } else { h }
    };

    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::default();
    let focal_length = 1.0;

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    println!("P3\n{image_width} {image_height}\n255");

    for j in 0..image_height {
        eprint!("\rscanlines remaining: {}", image_height - j);
        let _ = std::io::stderr().flush();
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;

            let r = Ray::new(&camera_center, &ray_direction);
            let col = ray_color(&r);

            write_color(&mut stdout(), &col);
        }
    }
    eprintln!("\rDone.                        ");
}
