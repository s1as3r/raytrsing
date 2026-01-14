use std::{cell::Ref, rc::Rc};

use super::{HitRecord, Hittable};
use crate::{
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: &Point3, radius: f64, mat: Rc<dyn Material>) -> Self {
        Self {
            center: *center,
            radius: f64::max(0.0, radius),
            mat: mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let oc = self.center - *r.origin();
        let a = r.direction().len_squared();
        // h = -2b
        let h = Vec3::dot(r.direction(), &oc);
        let c = oc.len_squared() - (self.radius * self.radius);
        let discriminant = (h * h) - (a * c);

        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = f64::sqrt(discriminant);

        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let mut rec = HitRecord {
            t: root,
            normal: Vec3::default(),
            p: r.at(root),
            mat: self.mat.clone(),
            front_face: false,
        };
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);

        Some(rec)
    }
}
