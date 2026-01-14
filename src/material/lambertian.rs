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
    rng: PCG32RNG,
}

impl Lambertian {
    pub fn new(albedo: &Color) -> Self {
        Self {
            albedo: *albedo,
            rng: PCG32RNG::default(),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&mut self, r_in: &Ray, rec: &HitRecord) -> (Color, Ray) {
        let scatter_direction = {
            let sd = rec.normal + Vec3::random_unit_vector(&mut self.rng);
            if sd.near_zero() { rec.normal } else { sd }
        };

        return (self.albedo, Ray::new(&rec.p, &scatter_direction));
    }
}
