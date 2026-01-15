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
    f64,
    io::{Write, stdout},
    rc::Rc,
};

use crate::{
    camera::Camera,
    color::{Color, write_color},
    hittable::{HitRecord, Hittable, list::HittableList, sphere::Sphere},
    interval::Interval,
    material::{Dielectric, Lambertian, Metal},
    ray::Ray,
    util::{lerp, rand::PCG32RNG},
    vec3::{Point3, Vec3},
};

fn main() {
    let mut world = HittableList::default();
    let radius = (f64::consts::PI / 4.0).cos();

    let mat_left = Rc::new(Lambertian::new(Color::new(0.0, 0.0, 1.0)));
    let mat_right = Rc::new(Lambertian::new(Color::new(1.0, 0.0, 0.0)));

    let sp_left = Sphere::new(Point3::new(-radius, 0.0, -1.0), radius, mat_left.clone());
    let sp_right = Sphere::new(Point3::new(radius, 0.0, -1.0), radius, mat_right.clone());

    world.add(&sp_left);
    world.add(&sp_right);

    let cam = Camera::new(
        16.0 / 10.0,
        400,
        100,
        50,
        90.0,
        Point3::new(-2.0, 2.0, 1.0),
        Point3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
    );
    cam.render(&world);
}
