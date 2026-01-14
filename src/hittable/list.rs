use super::{HitRecord, Hittable};

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
    fn hit(
        &self,
        r: &crate::ray::Ray,
        ray_tmin: f64,
        ray_tmax: f64,
        rec: &mut super::HitRecord,
    ) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_tmax;

        for &obj in &self.objects {
            if (obj.hit(r, ray_tmin, ray_tmax, &mut temp_rec)) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        return hit_anything;
    }
}
