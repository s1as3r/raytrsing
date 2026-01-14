pub mod lambertian;

use crate::{color::Color, hittable::HitRecord, ray::Ray, util::rand::PCG32RNG};

pub trait Material {
    // returns: attenuatoin, scattered ray
    fn scatter(&mut self, r_in: &Ray, rec: &HitRecord) -> (Color, Ray);
}
