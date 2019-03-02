use image;
use rand::Rng;

mod camera;
mod geometry;
mod materials;
mod objects;
mod render;
mod scene;

use camera::Camera;
use geometry::Vec3;
use materials::{Dielectric, Lambertian, Metal};
use objects::{ObjectList, Sphere};
use scene::Scene;

fn main() {
    // Initialise image size and quality
    let nx = 1200;
    let ny = 800;
    let ns = 1;

    // Initialise camera
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

    // Initialise world
    let world = random_world();

    // Initialise scene
    let scene = Scene::new(nx, ny, ns, camera, world);

    // Render scene
    let data = render::render(scene);

    // Generate image
    image::save_buffer("output.png", &data, nx, ny, image::RGB(8)).unwrap();
}

fn random_world() -> ObjectList {
    let mut world = ObjectList(vec![]);

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
