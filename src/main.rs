use image;
use rand::Rng;

mod camera;
mod materials;
mod objects;
mod ray;
mod render;
mod scene;
mod vector;

use camera::{Camera, Lens, Orientation};

use materials::Material;
use objects::Object;
use scene::Scene;
use vector::Vector;

fn main() {
    // Initialise image size and quality
    let nx = 1200;
    let ny = 800;
    let ns = 1;

    // Initialise camera
    let orientation = Orientation::new(
        Vector::new(13.0, 2.0, 3.0),
        Vector::new(0.0, 0.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );
    let lens = Lens::new(20.0, 0.1, 10.0);
    let aspect = f64::from(nx) / f64::from(ny);
    let camera = Camera::new(orientation, lens, aspect);

    // Initialise world
    let world = random_world();

    // Initialise scene
    let scene = Scene::new(nx, ny, ns, camera, world);

    // Render scene
    let data = render::render(scene);

    // Generate image
    image::save_buffer("output.png", &data, nx, ny, image::RGB(8)).unwrap();
}

fn random_world() -> Object {
    let mut object_list = vec![];

    // Floor
    object_list.push(Object::new_sphere(
        Vector::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::new_lambertian(Vector::new(0.5, 0.5, 0.5)),
    ));

    // Little balls
    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen_range(0.0, 1.0);
            let center = Vector::new(
                f64::from(a) + 0.9 * rng.gen_range(0.0, 1.0),
                0.2,
                f64::from(b) + 0.9 * rng.gen_range(0.0, 1.0),
            );
            if (center - Vector::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    object_list.push(Object::new_sphere(
                        center,
                        0.2,
                        Material::new_lambertian(Vector::new(
                            rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0),
                            rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0),
                            rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0),
                        )),
                    ));
                } else if choose_mat < 0.95 {
                    object_list.push(Object::new_sphere(
                        center,
                        0.2,
                        Material::new_metal(
                            Vector::new(
                                rng.gen_range(0.5, 1.0),
                                rng.gen_range(0.5, 1.0),
                                rng.gen_range(0.5, 1.0),
                            ),
                            0.5 * rng.gen_range(0.0, 0.5),
                        ),
                    ));
                } else {
                    object_list.push(Object::new_sphere(
                        center,
                        0.2,
                        Material::new_dielectric(1.5),
                    ))
                }
            }
        }
    }

    // Big balls
    object_list.push(Object::new_sphere(
        Vector::new(0.0, 1.0, 0.0),
        1.0,
        Material::new_dielectric(1.5),
    ));
    object_list.push(Object::new_sphere(
        Vector::new(-4.0, 1.0, 0.0),
        1.0,
        Material::new_lambertian(Vector::new(0.4, 0.2, 0.1)),
    ));
    object_list.push(Object::new_sphere(
        Vector::new(4.0, 1.0, 0.0),
        1.0,
        Material::new_metal(Vector::new(0.7, 0.6, 0.5), 0.0),
    ));

    Object::Multiple(object_list)
}
