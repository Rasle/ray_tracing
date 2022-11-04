use std::{
    fs::File,
    io::{LineWriter, Write},
    sync::mpsc::channel,
    thread,
    time::Instant,
    env,
};

use rayon::prelude::*;
use triple_buffer::TripleBuffer;

extern crate nalgebra as na;
use na::Vector3;

mod vec3;
use vec3::Vec3;

mod ray;
use ray::Ray;

mod sphere;
use sphere::Hittable;
use sphere::HittableList;
use sphere::Sphere;

mod camera;
use camera::Camera;

mod random;
use random::*;

mod material;

mod scene;
use scene::Scene;

mod render;
use render::*;

#[derive(Clone, Copy)]
enum RunningMode {
    File,
    Render,
}

fn main() {
    rayon::ThreadPoolBuilder::new().num_threads(12).build_global().unwrap();
    let args:Vec<String> = env::args().collect();
    let mode = 
        if args.len() < 2 {
            RunningMode::Render
        }
        else {
            let flag = &args[1];
            match flag.as_str() {
                "-File" => RunningMode::File,
                "-Render" => RunningMode::Render,
                _ => RunningMode::Render
            }
        };
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const WIDTH: u32 = 1200;
    const HEIGHT: u32 = (WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: i64 = 500;
    const MAX_DEPTH: i64 = 50;

    const FLAT_SIZE: usize = (WIDTH * HEIGHT) as usize;
    let pixel_data = vec![0; WIDTH as usize];

    let buf = TripleBuffer::new(&pixel_data);
    let (mut buffer_input, buffer_output) = buf.split();

    let (sender, receiver) = channel();
    let render = Render::new(buffer_output, receiver);

    thread::spawn(move || {
        let scene = Scene::one_weekend_scene(ASPECT_RATIO);
        // World
        let world = scene.objects;
        let camera = scene.camera;

        let file = File::create("image.ppm").expect("Failed to create file");
        let mut file = LineWriter::new(file);
        file.write_all(format!("P3\n{} {}\n255\n", WIDTH, HEIGHT).as_bytes())
            .expect("Failed to write data");

        let now = Instant::now();
        let mut pixel_data = vec![0; FLAT_SIZE];
        for (j, row) in pixel_data.chunks_mut(WIDTH as usize).enumerate().rev() {
            eprint!("\rScanlines remaining: {} ", j);
            row.par_iter_mut().enumerate().for_each(|(i, r)| {
                let mut pixel_color = Vec3::zeros();
                for _ in 0..SAMPLES_PER_PIXEL {
                    let u = (i as f64 + random_f64()) / ((WIDTH - 1) as f64);
                    let v = (j as f64 + random_f64()) / ((HEIGHT - 1) as f64);
                    let ray = camera.get_ray(u, v);
                    pixel_color += ray_color(ray, &world, MAX_DEPTH);
                }
                *r = set_color(pixel_color, SAMPLES_PER_PIXEL);
            });

            match mode {
                RunningMode::File => write_color_row(&mut file, row.iter()),
                RunningMode::Render => {
                    let input = buffer_input.input_buffer();
                    input.clear();
                    input.extend(row.iter());
                    buffer_input.publish();
                    sender.send(RenderStatus::Processing).unwrap();
                }
            }
        }

        eprint!("\nDone in {} seconds\n", now.elapsed().as_secs_f32());
        sender.send(RenderStatus::Done).unwrap();
    });

    let render_data: Vec<u32> = vec![255; FLAT_SIZE];
    render.render(render_data, WIDTH, HEIGHT);

    eprint!("Exited program");
}

fn write_color_row<'a>(file: &mut LineWriter<File>, colors: impl Iterator<Item = &'a u32>) {
    colors.for_each(|c| write_color(file, c))
}

fn write_color(file: &mut LineWriter<File>, color: &u32) {
    let r = color >> 16 & 0xFF;
    let g = color >> 8 & 0xFF;
    let b = color & 0xFF;

    let data = format!("{} {} {}\n", r, g, b);
    file.write_all(data.as_bytes())
        .expect("Failed to write data");
}

fn set_color(color: Vec3, samples_per_pixel: i64) -> u32 {
    let scale = 1.0 / samples_per_pixel as f64;
    let r = (color.x * scale).sqrt();
    let g = (color.y * scale).sqrt();
    let b = (color.z * scale).sqrt();

    let ur = (256.0 * clamp(r, 0.0, 0.999)) as u32;
    let ug = (256.0 * clamp(g, 0.0, 0.999)) as u32;
    let ub = (256.0 * clamp(b, 0.0, 0.999)) as u32;

    (255 << 24) + (ur << 16) + (ug << 8) + ub
}

fn ray_color(r: Ray, world: &HittableList, depth: i64) -> Vec3 {
    if depth <= 0 {
        Vec3::zeros()
    } else if let Some(hit) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = hit.mat.scatter(&r, &hit) {
            attenuation * ray_color(scattered, world, depth - 1)
        } else {
            Vec3::zeros()
        }
    } else {
        let unit_direction = Vec3::unit_vector(r.direction);
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Vec3::ones() + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

// fn test_scene() -> HittableList {
//     let mut world = HittableList::new();

//     let material_ground = Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
//     let material_center =  Rc::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
//     let material_left =  Rc::new(Dielectric::new(1.5));
//     let material_right =  Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0));

//     world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
//     world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, material_center)));
//     world.add(Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
//     world.add(Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right)));

//     world
// }

// fn test_camera(aspect_ratio : f64) -> Camera {
//     let lookfrom = Vec3::new(3.0, 3.0, 2.0);
//     let lookat = Vec3::new(0.0, 0.0, -1.0);
//     let vup = Vec3::new(0.0, 1.0, 0.0);
//     let dist_to_focus = (lookfrom - lookat).length();
//     let aperture = 2.0;
//     Camera::new(lookfrom, lookat, vup, 90.0, aspect_ratio, aperture, dist_to_focus)
// }

#[inline(always)]
fn clamp(x: f64, min: f64, max: f64) -> f64 {
    x.max(min).min(max)
}
