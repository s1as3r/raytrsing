use std::io::{Write, stdout};

use crate::{
    color::{Color, write_color},
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
    util,
    vec3::{Point3, Vec3},
};

pub struct Camera {
    aspect_ratio: f64,
    image_width: i32,
    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i32) -> Self {
        let image_height = {
            let h = (image_width as f64 / aspect_ratio) as i32;
            if h < 1 { 1 } else { h }
        };

        let center = Point3::default();

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&self, world: &dyn Hittable) {
        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprint!("\rscanlines remaining: {}", self.image_height - j);
            let _ = std::io::stderr().flush();
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc
                    + (i as f64 * self.pixel_delta_u)
                    + (j as f64 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;

                let r = Ray::new(&self.center, &ray_direction);
                let col = self.ray_color(&r, world);

                write_color(&mut stdout(), &col);
            }
        }
        eprintln!("\rDone.                        ");
    }

    fn ray_color(&self, r: &Ray, world: &dyn Hittable) -> Color {
        let mut rec = HitRecord::default();

        if (world.hit(r, &Interval::new(0.0, f64::INFINITY), &mut rec)) {
            return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
        }

        let unit_direction = Vec3::unit_vector(r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        util::lerp(Color::new(1.0, 1.0, 1.0), Color::new(0.5, 0.7, 1.0), a)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(16.0 / 10.0, 100)
    }
}
