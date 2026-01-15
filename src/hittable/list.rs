use std::rc::Rc;

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
};

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

#[allow(dead_code)]
impl HittableList {
    pub fn new(object: Rc<dyn Hittable>) -> Self {
        Self {
            objects: vec![object],
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut closest_so_far = ray_t.max;
        let mut rec = None;

        for obj in &self.objects {
            if let Some(t_rec) = obj.hit(r, &Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = t_rec.t;
                rec = Some(t_rec)
            }
        }

        rec
    }
}
