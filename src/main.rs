#![allow(dead_code, unused)]
mod camera;
mod color;
mod hittable;
mod interval;
mod ray;
mod util;
mod vec3;

use std::io::{Write, stdout};

use crate::{
    camera::Camera,
    color::{Color, write_color},
    hittable::{HitRecord, Hittable, list::HittableList, sphere::Sphere},
    interval::Interval,
    ray::Ray,
    util::lerp,
    vec3::{Point3, Vec3},
};

fn main() {
    let mut world = HittableList::default();
    let sp1 = Sphere::new(&Point3::new(0.0, 0.0, -1.0), 0.5);
    let sp2 = Sphere::new(&Point3::new(0.0, -100.5, -1.0), 100.0);
    world.add(&sp2);
    world.add(&sp1);

    let cam = Camera::new(16.0 / 10.0, 400);
    cam.render(&world);
}
