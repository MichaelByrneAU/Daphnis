mod hitable;
mod ray;
mod sphere;
mod vec3;

use hitable::{Hitable, HitableList};
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;

fn main() {
    let nx = 200;
    let ny = 100;

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

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

    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = f64::from(i) / f64::from(nx);
            let v = f64::from(j) / f64::from(ny);
            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);

            let col = colour(&r, &world);
            let ir = (255.99 * col[0]) as i32;
            let ig = (255.99 * col[1]) as i32;
            let ib = (255.99 * col[2]) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}

fn colour(r: &Ray, world: &Hitable) -> Vec3 {
    let hit = world.hit(r, 0.0, std::f64::MAX);

    match hit {
        Some(hit_record) => {
            let normal = hit_record.normal;
            0.5 * Vec3::new(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0)
        }
        None => {
            let unit_direction = r.direction.unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}
