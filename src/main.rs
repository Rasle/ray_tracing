use std::{io::{Write, LineWriter},
    fs::File,
    sync::{Arc, RwLock, mpsc::channel},
    thread,
    rc::Rc};

use minifb::{Key, Window, WindowOptions};

mod vec3;
use vec3::Vec3;

mod ray;
use ray::Ray;

mod sphere;
use sphere::Sphere;
use sphere::Hittable;
use sphere::HittableList;

mod camera;
use camera::Camera;

mod random;
use random::*;

mod material;
use material::*;

enum RenderStatus {
    Processing,
    Done
}

fn main() {
    // Image
    const ASPECT_RATIO : f64 = 3.0 / 2.0;
    const WIDTH : u32 = 1200;
    const HEIGHT : u32 = (WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL : i64 = 500;
    const MAX_DEPTH : i64 = 50;

    const FLAT_SIZE : usize = (WIDTH * HEIGHT) as usize;
    let pixel_data = vec![0; FLAT_SIZE];

    let lock = Arc::new(RwLock::new(pixel_data));
    let c_lock = Arc::clone(&lock);

    let (sender, receiver) = channel();

    thread::spawn(move || {
        // World
        let world = random_scene();

        // Camera
        let lookfrom = Vec3::new(13.0, 2.0, 3.0);
        let lookat = Vec3::new(0.0, 0.0, 0.0);
        let vup = Vec3::new(0.0, 1.0, 0.0);
        let dist_to_focus = 10.0;
        let aperture = 0.1;
        let camera = Camera::new(lookfrom, lookat, vup, 20.0, ASPECT_RATIO, aperture, dist_to_focus);

        let file = File::create("image.ppm").expect("Failed to create file");
        let mut file = LineWriter::new(file);
        file.write_all(format!("P3\n{} {}\n255\n", WIDTH, HEIGHT).as_bytes()).expect("Failed to write data");
        for j in (0..HEIGHT).rev() {
            eprint!("\rScanlines remaining: {} ", j);
            for i in 0..WIDTH {
                let mut pixel_color = Vec3::zeros();
                for _s in 0..SAMPLES_PER_PIXEL {
                    let u = (i as f64 + random_f64()) / ((WIDTH - 1) as f64);
                    let v = (j as f64 + random_f64()) / ((HEIGHT - 1) as f64);
                    let ray = camera.get_ray(u, v);
                    pixel_color += ray_color(ray, &world, MAX_DEPTH);
                }
                write_color(&mut file, pixel_color, SAMPLES_PER_PIXEL);
                let index = ((HEIGHT - j - 1) * WIDTH + i) as usize;
                let mut write = lock.write().unwrap();
                set_color(&mut *write, index, pixel_color, SAMPLES_PER_PIXEL);
                drop(write);
            }
            sender.send(RenderStatus::Processing).unwrap();
        }

        eprint!("\nDone.\n");
        sender.send(RenderStatus::Done).unwrap();
    });

    let options = WindowOptions { borderless : false , title : true, resize : true, scale : minifb::Scale::X1, scale_mode : minifb::ScaleMode::Center, topmost : false, transparency : false, none : false};
    let mut window = Window::new(
        "Ray Tracer - ESC to exit",
        WIDTH as usize,
        HEIGHT as usize,
        options,
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let progress = receiver.try_recv();
        match progress {
            Ok(status) => {
                match status {
                    RenderStatus::Processing => {
                        let read = c_lock.read().unwrap();
                        window
                            .update_with_buffer(& *read, WIDTH as usize, HEIGHT as usize)
                            .unwrap();
                        drop(read);
                    },
                    RenderStatus::Done => {
                        window.update()
                    },
                }
            },
            Err(_) => {
                window.update()
            },
        }
    }

    eprint!("Exited program");
}

fn write_color(file : &mut LineWriter<File>, color : Vec3, samples_per_pixel : i64) {
    let scale = 1.0 / samples_per_pixel as f64;
    let r = (color.x * scale).sqrt();
    let g = (color.y * scale).sqrt();
    let b = (color.z * scale).sqrt();

    let ir = (256.0 * clamp(r, 0.0, 0.999)) as i64;
    let ig = (256.0 * clamp(g, 0.0, 0.999)) as i64;
    let ib = (256.0 * clamp(b, 0.0, 0.999)) as i64;

    let data = format!("{} {} {}\n", ir, ig, ib);
    file.write_all(data.as_bytes()).expect("Failed to write data");
}

fn set_color (data : &mut Vec<u32>, index : usize, color : Vec3, samples_per_pixel : i64) {
    let scale = 1.0 / samples_per_pixel as f64;
    let r = (color.x * scale).sqrt();
    let g = (color.y * scale).sqrt();
    let b = (color.z * scale).sqrt();

    let ur = (256.0 * clamp(r, 0.0, 0.999)) as u32;
    let ug = (256.0 * clamp(g, 0.0, 0.999)) as u32;
    let ub = (256.0 * clamp(b, 0.0, 0.999)) as u32;

    data[index] = (255 << 24) + (ur << 16) + (ug << 8) + ub;
}

fn ray_color(r : Ray, world : &HittableList, depth : i64) -> Vec3 {
    if depth <= 0 {
        Vec3::zeros()
    }
    else if let Some(hit) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = hit.mat.scatter(&r, &hit){
            attenuation * ray_color(scattered, world, depth - 1)
        }
        else {
            Vec3::zeros()
        }
    }
    else {
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

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Rc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64();
            let center = Vec3::new(a as f64 + 0.9 * random_f64(), 0.2, b as f64 + 0.9 * random_f64());

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<dyn Material> = 
                    if choose_mat < 0.8 {
                        let albedo = Vec3::random() * Vec3::random();
                        Rc::new(Lambertian::new(albedo))
                    }
                    else if choose_mat < 0.95 {
                        let albedo = Vec3::random_range(0.5, 1.0);
                        let fuzz = random_f64_range(0.0, 0.5);
                        Rc::new(Metal::new(albedo, fuzz))
                    }
                    else {
                        Rc::new(Dielectric::new(1.5))
                    };
                world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = Rc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Rc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material3)));

    world
}

#[inline(always)]
fn clamp(x : f64, min : f64, max : f64) -> f64 {
    x.max(min).min(max)
}