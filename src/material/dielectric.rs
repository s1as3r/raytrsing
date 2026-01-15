use crate::{
    color::Color,
    hittable::HitRecord,
    material::Material,
    ray::Ray,
    util::rand::PCG32RNG,
    vec3::Vec3,
};

pub struct Dielectric {
    refractive_index: f64,
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Self {
        Self { refractive_index }
    }

    fn reflectance(cosine: f64, refractive_index: f64) -> f64 {
        // Schlick's approximation
        let r0 = ((1.0 - refractive_index) / (1.0 + refractive_index)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: &mut PCG32RNG) -> Option<(Color, Ray)> {
        let ri = if rec.front_face {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };

        let unit_diection = r_in.direction().unit_vector();
        let cos_theta = Vec3::dot(&-unit_diection, &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        // handle total internal reflection
        let cannot_refract = (ri * sin_theta) > 1.0;
        let direction = if cannot_refract || (Self::reflectance(cos_theta, ri) > rng.random_f64()) {
            Vec3::reflect(&unit_diection, &rec.normal)
        } else {
            Vec3::refract(&unit_diection, &rec.normal, ri)
        };

        Some((Color::new(1.0, 1.0, 1.0), Ray::new(rec.p, direction)))
    }
}
