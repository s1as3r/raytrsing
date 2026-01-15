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
    let mat_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let mat_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let mat_left = Rc::new(Dielectric::new(1.50));
    let mat_bubble = Rc::new(Dielectric::new(1.00 / 1.50));
    let mat_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    let sp_ground = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, mat_ground.clone());
    let sp_center = Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, mat_center.clone());
    let sp_left = Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, mat_left.clone());
    let sp_bubble = Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.4, mat_bubble.clone());
    let sp_right = Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, mat_right.clone());
    world.add(&sp_ground);
    world.add(&sp_left);
    world.add(&sp_right);
    world.add(&sp_bubble);
    world.add(&sp_center);

    let cam = Camera::new(
        16.0 / 10.0,
        400,
        100,
        50,
        20.0,
        Point3::new(-2.0, 2.0, 1.0),
        Point3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
    );
    cam.render(&world);
}
