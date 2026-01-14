#![allow(dead_code, unused)]
mod camera;
mod color;
mod hittable;
mod interval;
mod material;
mod ray;
mod util;
mod vec3;

use std::{
    io::{Write, stdout},
    rc::Rc,
};

use crate::{
    camera::Camera,
    color::{Color, write_color},
    hittable::{HitRecord, Hittable, list::HittableList, sphere::Sphere},
    interval::Interval,
    material::lambertian::Lambertian,
    ray::Ray,
    util::{lerp, rand::PCG32RNG},
    vec3::{Point3, Vec3},
};

fn main() {
    let mut world = HittableList::default();
    let mut rng = PCG32RNG::default();
    let lamb = Rc::new(Lambertian::new(&Vec3::default()));
    let sp1 = Sphere::new(&Point3::new(0.0, 0.0, -1.0), 0.5, lamb.clone());
    let sp2 = Sphere::new(&Point3::new(0.0, -100.5, -1.0), 100.0, lamb.clone());
    world.add(&sp2);
    world.add(&sp1);

    let cam = Camera::new(16.0 / 10.0, 400, 100, 50);
    cam.render(&world);
}
