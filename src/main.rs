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
    material::{Dielectric, Lambertian, Material, Metal},
    ray::Ray,
    util::{lerp, rand::PCG32RNG},
    vec3::{Point3, Vec3},
};

fn main() {
    let mut world = HittableList::default();
    let mut rng = PCG32RNG::default();

    let mat_ground = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let sp_ground = Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        mat_ground.clone(),
    ));
    world.add(sp_ground.clone());

    let mut choose_mat: f64;
    let mut center: Point3;
    for a in -11..11 {
        for b in -11..11 {
            let a = a as f64;
            let b = b as f64;

            choose_mat = rng.random_f64();
            center = Point3::new(a + 0.9 * rng.random_f64(), 0.2, b + 0.9 * rng.random_f64());

            if (center - Point3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                let sphere_mat: Rc<dyn Material> = {
                    if choose_mat < 0.8 {
                        Rc::new(Lambertian::new(
                            Color::random(&mut rng) * Color::random(&mut rng),
                        ))
                    } else if choose_mat < 0.95 {
                        Rc::new(Metal::new(
                            Color::random_bounded(&mut rng, 0.5, 1.0),
                            rng.random_bounded_f64(0.0, 0.5),
                        ))
                    } else {
                        Rc::new(Dielectric::new(1.5))
                    }
                };
                let sp = Rc::new(Sphere::new(center, 0.2, sphere_mat.clone()));
                world.add(sp.clone());
            }
        }
    }

    let mat1 = Rc::new(Dielectric::new(1.5));
    let sp1 = Rc::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1.clone()));
    world.add(sp1.clone());

    let mat2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let sp2 = Rc::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2.clone()));
    world.add(sp2.clone());

    let mat3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    let sp3 = Rc::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3.clone()));
    world.add(sp3.clone());

    let cam = Camera::new(
        16.0 / 10.0,
        1200,
        10,
        50,
        20.0,
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.6,
        10.0,
    );
    cam.render(&world, &mut rng);
}
