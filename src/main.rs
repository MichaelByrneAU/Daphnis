use rand::Rng;

mod camera;
mod hitable;
mod material;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use hitable::{Hitable, HitableList};
use material::{Dielectric, Lambertian, Metal};
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;

fn main() {
    let nx = 300;
    let ny = 200;
    let ns = 11;

    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;

    let camera = Camera::new(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        f64::from(nx) / f64::from(ny),
        0.1,
        dist_to_focus,
    );

    let world = random_scene();

    let mut rng = rand::thread_rng();

    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    for j in (0..ny).rev() {
        dbg!(j);
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u = (f64::from(i) + rng.gen_range(0.0, 1.0)) / f64::from(nx);
                let v = (f64::from(j) + rng.gen_range(0.0, 1.0)) / f64::from(ny);
                let r = camera.get_ray(u, v);
                col += colour(&r, &world, 0);
            }
            col /= f64::from(ns);
            col = Vec3::new(col[0].sqrt(), col[1].sqrt(), col[2].sqrt());
            let ir = (255.99 * col[0]) as i32;
            let ig = (255.99 * col[1]) as i32;
            let ib = (255.99 * col[2]) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}

fn colour(r: &Ray, world: &Hitable, depth: i32) -> Vec3 {
    let hit = world.hit(r, 0.0001, std::f64::MAX);

    match hit {
        Some(hit_record) => {
            if depth < 50 {
                let scatter = hit_record.material.scatter(r, &hit_record);
                scatter
                    .attenuation
                    .elem_wise_mul(colour(&scatter.ray, world, depth + 1))
            } else {
                Vec3::new(0.0, 0.0, 0.0)
            }
        }
        None => {
            let unit_direction = r.direction.unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}

fn random_scene() -> HitableList {
    let n = 500;
    let mut world = HitableList(vec![]);

    // Floor
    world.0.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Box::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))),
    )));

    // Little balls
    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen_range(0.0, 1.0);
            let center = Vec3::new(
                f64::from(a) + 0.9 * rng.gen_range(0.0, 1.0),
                0.2,
                f64::from(b) + 0.9 * rng.gen_range(0.0, 1.0),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    world.0.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Box::new(Lambertian::new(Vec3::new(
                            rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0),
                            rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0),
                            rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0),
                        ))),
                    )));
                } else if choose_mat < 0.95 {
                    world.0.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Box::new(Metal::new(
                            Vec3::new(
                                rng.gen_range(0.5, 1.0),
                                rng.gen_range(0.5, 1.0),
                                rng.gen_range(0.5, 1.0),
                            ),
                            0.5 * rng.gen_range(0.0, 0.5),
                        )),
                    )));
                } else {
                    world.0.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Box::new(Dielectric::new(1.5)),
                    )))
                }
            }
        }
    }

    // Big balls
    world.0.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Box::new(Dielectric::new(1.5)),
    )));
    world.0.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Box::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))),
    )));
    world.0.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Box::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
    )));

    world
}
