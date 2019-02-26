mod ray;
mod vec3;

use ray::Ray;
use vec3::Vec3;

fn main() {
    let nx = 200;
    let ny = 100;

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = f64::from(i) / f64::from(nx);
            let v = f64::from(j) / f64::from(ny);
            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let col = colour(&r);

            let ir = (255.99 * col[0]) as i32;
            let ig = (255.99 * col[1]) as i32;
            let ib = (255.99 * col[2]) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}

fn hit_sphere(center: &Vec3, radius: f64, r: &Ray) -> bool {
    let oc = r.origin - *center;

    let a = r.direction.dot(r.direction);
    let b = 2.0 * oc.dot(r.direction);
    let c = oc.dot(oc) - radius * radius;

    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}

fn colour(r: &Ray) -> Vec3 {
    if hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, r) {
        Vec3::new(1.0, 0.0, 0.0)
    } else {
        let unit_direction = r.direction.unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}
