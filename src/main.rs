use rand::Rng;

mod camera;
mod hitable;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use hitable::{Hitable, HitableList};
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);

    let camera = Camera::new(origin, lower_left_corner, horizontal, vertical);

    let spheres = vec![
        Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0),
    ];

    let world = HitableList(
        spheres
            .into_iter()
            .map(|s| Box::new(s) as Box<dyn Hitable>)
            .collect(),
    );

    let mut rng = rand::thread_rng();

    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u = (f64::from(i) + rng.gen_range(0.0, 1.0)) / f64::from(nx);
                let v = (f64::from(j) + rng.gen_range(0.0, 1.0)) / f64::from(ny);
                let r = camera.get_ray(u, v);
                col += colour(&r, &world);
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

fn random_in_unit_sphere() -> Vec3 {
    let mut point = Vec3::new(std::f64::MAX, std::f64::MAX, std::f64::MAX);
    let mut rng = rand::thread_rng();
    loop {
        if point.squared_length() < 1.0 {
            break;
        }
        point = Vec3::new(
            rng.gen_range(-0.5, 0.5),
            rng.gen_range(-0.5, 0.5),
            rng.gen_range(-0.5, 0.5),
        );
    }
    point
}

fn colour(r: &Ray, world: &Hitable) -> Vec3 {
    let hit = world.hit(r, 0.0001, std::f64::MAX);

    match hit {
        Some(hit_record) => {
            let target = hit_record.p + hit_record.normal + random_in_unit_sphere();
            0.5 * colour(&Ray::new(hit_record.p, target - hit_record.p), world)
        }
        None => {
            let unit_direction = r.direction.unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}
