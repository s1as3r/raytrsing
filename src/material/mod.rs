mod lambertian;
mod metal;

use crate::{color::Color, hittable::HitRecord, ray::Ray, util::rand::PCG32RNG};

pub use lambertian::Lambertian;
pub use metal::Metal;

pub trait Material {
    // returns: attenuatoin, scattered ray
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: &mut PCG32RNG) -> Option<(Color, Ray)>;
}
