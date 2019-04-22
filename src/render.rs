use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;

use crate::objects::Object;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::vector::Vector;

pub fn render(scene: Scene) -> Vec<u8> {
    let rays = (scene.width * scene.height * scene.samples) as usize;

    // Initialise progress bar
    let pb = ProgressBar::new(rays as u64);
    let pb_style = ProgressStyle::default_bar()
        .template("[{eta_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
        .progress_chars("##-");
    pb.set_style(pb_style);

    // Rendering loop
    let mut data = Vec::with_capacity(rays);
    let mut rng = rand::thread_rng();
    for j in (0..scene.height).rev() {
        for i in 0..scene.width {
            let mut col = Vector::new(0.0, 0.0, 0.0);
            for _ in 0..scene.samples {
                let u = (f64::from(i) + rng.gen_range(0.0, 1.0)) / f64::from(scene.width);
                let v = (f64::from(j) + rng.gen_range(0.0, 1.0)) / f64::from(scene.height);
                let r = scene.camera.get_ray(u, v);
                col += colour(&r, &scene.world, 0);
            }
            col /= f64::from(scene.samples);
            col = Vector::new(col[0].sqrt(), col[1].sqrt(), col[2].sqrt());
            let r = (255.99 * col[0]) as u8;
            let g = (255.99 * col[1]) as u8;
            let b = (255.99 * col[2]) as u8;

            data.push(r);
            data.push(g);
            data.push(b);

            // Update progress bar
            pb.inc(1);
        }
    }

    pb.finish();
    data
}

fn colour(r: &Ray, world: &Object, depth: i32) -> Vector {
    let hit = world.hit(r, 0.0001, std::f64::MAX);

    match hit {
        Some(hit_record) => {
            if depth < 50 {
                let scatter = hit_record.material.scatter(r, &hit_record);
                scatter.attenuation * colour(&scatter.ray, world, depth + 1)
            } else {
                Vector::new(0.0, 0.0, 0.0)
            }
        }
        None => {
            let unit_direction = r.direction.unit_vector();
            let t = 0.5 * (unit_direction.y + 1.0);
            (1.0 - t) * Vector::new(1.0, 1.0, 1.0) + t * Vector::new(0.5, 0.7, 1.0)
        }
    }
}
