use crate::{
    color::Color,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    util::rand::PCG32RNG,
    vec3::Vec3,
};

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: &mut PCG32RNG) -> Option<(Color, Ray)> {
        let scatter_direction = {
            let sd = rec.normal + Vec3::random_unit_vector(rng);
            if sd.near_zero() { rec.normal } else { sd }
        };

        return Some((self.albedo, Ray::new(rec.p, scatter_direction)));
    }
}
