use std::io::{self, Write, stdout};

use crate::{
    color::{Color, write_color},
    hittable::Hittable,
    interval::Interval,
    ray::Ray,
    util::{self, rand::PCG32RNG},
    vec3::{Point3, Vec3},
};

#[allow(dead_code)]
pub struct Camera {
    aspect_ratio: f64,
    image_width: i32,
    samples_per_pixel: i32,
    max_depth: i32,

    vfov: f64,
    lookfrom: Point3,
    lookat: Point3,
    vup: Vec3, // camera-relative "up" direction

    defocus_angle: f64,
    focus_dist: f64,

    image_height: i32,
    pixel_samples_scale: f64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,

    // camera frame basis vectors
    u: Vec3,
    v: Vec3,
    w: Vec3,

    defocus_disk_u: Vec3, // horizontal
    defocus_disk_v: Vec3, // vertical
}

impl Camera {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        aspect_ratio: f64,
        image_width: i32,
        samples_per_pixel: i32,
        max_depth: i32,
        vfov: f64,
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        defocus_angle: f64,
        focus_dist: f64,
    ) -> Self {
        let image_height = {
            let h = (image_width as f64 / aspect_ratio) as i32;
            if h < 1 { 1 } else { h }
        };

        let center = lookfrom;

        let theta = util::deg_to_rad(vfov);
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let w = (lookfrom - lookat).unit_vector();
        let u = Vec3::cross(&vup, &w).unit_vector();
        let v = Vec3::cross(&w, &u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left =
            center - (focus_dist * w) - (viewport_u / 2.0) - (viewport_v / 2.0);
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_radius = focus_dist * util::deg_to_rad(defocus_angle / 2.0).tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

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
            vfov,
            lookfrom,
            lookat,
            vup,
            u,
            v,
            w,
            defocus_angle,
            focus_dist,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub fn render(&self, world: &dyn Hittable, rng: &mut PCG32RNG) -> io::Result<()> {
        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprint!("\rscanlines remaining: {:>04}", self.image_height - j);
            let _ = std::io::stderr().flush();
            for i in 0..self.image_width {
                let mut pixel_color = Color::default();
                let mut r: Ray;
                for _ in 0..self.samples_per_pixel {
                    r = self.get_ray(i, j, rng);
                    pixel_color += self.ray_color(&r, self.max_depth, world, rng);
                }

                write_color(&mut stdout(), &(pixel_color * self.pixel_samples_scale))?;
            }
        }
        eprintln!("\rDone.                        ");
        Ok(())
    }

    fn ray_color(&self, r: &Ray, depth: i32, world: &dyn Hittable, rng: &mut PCG32RNG) -> Color {
        if depth <= 0 {
            return Color::default();
        }

        // min 0.001: we want to ignore hits that are very close to the intersection
        // point because of floating point imprecision
        // SEE: shadow acne
        if let Some(rec) = world.hit(r, &Interval::new(0.001, f64::INFINITY)) {
            if let Some((attenuation, scattered)) = rec.mat.scatter(r, &rec, rng) {
                return attenuation * self.ray_color(&scattered, depth - 1, world, rng);
            }
            return Color::default();
        }

        let unit_direction = r.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        util::lerp(Color::new(1.0, 1.0, 1.0), Color::new(0.5, 0.7, 1.0), a)
    }

    fn get_ray(&self, i: i32, j: i32, rng: &mut PCG32RNG) -> Ray {
        // constructs a camera ray form the defocus disk and directed at a randomly sampled point
        // around the pixel location i, j
        let offset = self.sample_square(rng);
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample(rng)
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square(&self, rng: &mut PCG32RNG) -> Vec3 {
        Vec3::new(rng.random_f64() - 0.5, rng.random_f64() - 0.5, 0.0)
    }

    fn defocus_disk_sample(&self, rng: &mut PCG32RNG) -> Point3 {
        let p = Point3::random_in_unit_disk(rng);
        self.center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(
            16.0 / 10.0,
            100,
            5,
            10,
            90.0,
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 0.0, -1.0),
            Vec3::new(1.0, 1.0, 0.0),
            0.0,
            10.0,
        )
    }
}
