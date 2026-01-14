use std::{
    io::{Write, stdout},
    rc::Rc,
};

use crate::{
    color::{Color, write_color},
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
    util::{
        self,
        rand::{self, PCG32RNG},
    },
    vec3::{Point3, Vec3},
};

pub struct Camera {
    aspect_ratio: f64,
    image_width: i32,
    samples_per_pixel: i32,
    max_depth: i32,

    image_height: i32,
    pixel_samples_scale: f64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: i32,
        samples_per_pixel: i32,
        max_depth: i32,
    ) -> Self {
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
        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;

        Self {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            pixel_samples_scale,
        }
    }

    pub fn render(&self, world: &dyn Hittable) {
        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        let mut rng = PCG32RNG::default();
        for j in 0..self.image_height {
            eprint!("\rscanlines remaining: {:>04}", self.image_height - j);
            let _ = std::io::stderr().flush();
            for i in 0..self.image_width {
                let mut pixel_color = Color::default();
                let mut r: Ray;
                for sample in 0..self.samples_per_pixel {
                    r = self.get_ray(i, j, &mut rng);
                    pixel_color += self.ray_color(&r, self.max_depth, world, &mut rng);
                }

                write_color(&mut stdout(), &(pixel_color * self.pixel_samples_scale));
            }
        }
        eprintln!("\rDone.                        ");
    }

    fn ray_color(&self, r: &Ray, depth: i32, world: &dyn Hittable, rng: &mut PCG32RNG) -> Color {
        if depth <= 0 {
            return Color::default();
        }

        // min 0.001: we want to ignore hits that are very close to the intersection
        // point because of floating point imprecision
        // SEE: shadow acne
        if let Some(mut rec) = world.hit(r, &Interval::new(0.001, f64::INFINITY)) {
            let (attenuation, scattered) = rec.mat.scatter(r, &rec, rng);
            return attenuation * self.ray_color(&scattered, depth - 1, world, rng);
        }

        let unit_direction = Vec3::unit_vector(r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        util::lerp(Color::new(1.0, 1.0, 1.0), Color::new(0.5, 0.7, 1.0), a)
    }

    fn get_ray(&self, i: i32, j: i32, rng: &mut PCG32RNG) -> Ray {
        // constructs a camera ray form the origin and directed at randomly sampled points
        // around the pixel location i, j
        let offset = self.sample_square(rng);
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(&ray_origin, &ray_direction)
    }

    fn sample_square(&self, rng: &mut PCG32RNG) -> Vec3 {
        Vec3::new(rng.random_f64() - 0.5, rng.random_f64() - 0.5, 0.0)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(16.0 / 10.0, 100, 5, 10)
    }
}
