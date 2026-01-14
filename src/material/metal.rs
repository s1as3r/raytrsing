use crate::{
    color::Color, hittable::HitRecord, material::Material, ray::Ray, util::rand::PCG32RNG,
    vec3::Vec3,
};

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, _rng: &mut PCG32RNG) -> (Color, Ray) {
        let reflected = Vec3::reflect(r_in.direction(), &rec.normal);
        (self.albedo, Ray::new(rec.p, reflected))
    }
}
