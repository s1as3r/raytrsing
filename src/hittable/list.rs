use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
};

#[derive(Default)]
pub struct HittableList<'a> {
    pub objects: Vec<&'a dyn Hittable>,
}

impl<'a> HittableList<'a> {
    pub fn new(object: &'a dyn Hittable) -> Self {
        Self {
            objects: vec![object],
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, object: &'a dyn Hittable) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList<'_> {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;
        let mut rec = None;

        for &obj in &self.objects {
            if let Some(t_rec) = obj.hit(r, ray_t) {
                hit_anything = true;
                closest_so_far = t_rec.t;
                rec = Some(t_rec)
            }
        }

        return rec;
    }
}
