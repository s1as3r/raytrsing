#![allow(dead_code, unused)]
mod color;
mod hittable;
mod ray;
mod util;
mod vec3;

use std::io::{Write, stdout};

use crate::{
    color::{Color, write_color},
    hittable::{HitRecord, Hittable, list::HittableList, sphere::Sphere},
    ray::Ray,
    util::lerp,
    vec3::{Point3, Vec3},
};

fn main() {
    let aspect_ratio = 16.0 / 10.0;

    let image_width = 400;
    let image_height = {
        let h = (image_width as f64 / aspect_ratio) as i32;
        if h < 1 { 1 } else { h }
    };

    let mut world = HittableList::default();
    let sp1 = Sphere::new(&Point3::new(0.0, 0.0, -1.0), 0.5);
    let sp2 = Sphere::new(&Point3::new(0.0, -100.5, -1.0), 100.0);
    world.add(&sp2);
    world.add(&sp1);

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
            let col = ray_color(&r, &world);

            write_color(&mut stdout(), &col);
        }
    }
    eprintln!("\rDone.                        ");
}

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let mut hr = HitRecord::default();
    if Sphere::new(center, radius).hit(r, 0.0, 1.0, &mut hr) {
        hr.t
    } else {
        -1.0
    }
}

fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    let mut rec = HitRecord::default();
    if (world.hit(r, 0.0, f64::INFINITY, &mut rec)) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }
    let unit_direction = Vec3::unit_vector(r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    lerp(Color::new(1.0, 1.0, 1.0), Color::new(0.5, 0.7, 1.0), a)
}
