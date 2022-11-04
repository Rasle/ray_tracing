use crate::random::*;
use crate::Vec3;
use crate::HittableList;
use crate::Camera;
use crate::material::*;
use crate::Sphere;

pub struct Scene {
    pub objects : HittableList,
    pub camera : Camera
}

impl Scene {
    pub fn one_weekend_scene(aspect_ratio: f64) -> Scene {
        Scene {
            objects: Self::random_scene(),
            camera: Self::get_camera(aspect_ratio)
        }
    }

    fn get_camera(aspect_ratio: f64) -> Camera {
        let lookfrom = Vec3::new(13.0, 2.0, 3.0);
        let lookat = Vec3::new(0.0, 0.0, 0.0);
        let vup = Vec3::new(0.0, 1.0, 0.0);
        let focus_dist = 10.0;
        let aperture = 0.1;
        let vfov = 20.0;
        Camera::new(
            lookfrom,
            lookat,
            vup,
            vfov,
            aspect_ratio,
            aperture,
            focus_dist,
        )
    }
    fn random_scene() -> HittableList {
        let mut world = HittableList::new();

        let ground_material = Lambertian::new(Vec3::new(0.5, 0.5, 0.5));
        world.add(Box::new(Sphere::new(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            ground_material,
        )));

        for a in -11..11 {
            for b in -11..11 {
                let choose_mat = random_f64();
                let center = Vec3::new(
                    a as f64 + 0.9 * random_f64(),
                    0.2,
                    b as f64 + 0.9 * random_f64(),
                );

                if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                    if choose_mat < 0.8 {
                        let albedo = Vec3::random() * Vec3::random();
                        world.add(Box::new(Sphere::new(center, 0.2, Lambertian::new(albedo))));
                    } else if choose_mat < 0.95 {
                        let albedo = Vec3::random_range(0.5, 1.0);
                        let fuzz = random_f64_range(0.0, 0.5);
                        world.add(Box::new(Sphere::new(center, 0.2, Metal::new(albedo, fuzz))));
                    } else {
                        world.add(Box::new(Sphere::new(center, 0.2, Dielectric::new(1.5))));
                    };
                }
            }
        }

        let material1 = Dielectric::new(1.5);
        world.add(Box::new(Sphere::new(
            Vec3::new(0.0, 1.0, 0.0),
            1.0,
            material1,
        )));

        let material2 = Lambertian::new(Vec3::new(0.4, 0.2, 0.1));
        world.add(Box::new(Sphere::new(
            Vec3::new(-4.0, 1.0, 0.0),
            1.0,
            material2,
        )));

        let material3 = Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0);
        world.add(Box::new(Sphere::new(
            Vec3::new(4.0, 1.0, 0.0),
            1.0,
            material3,
        )));

        world
    }
}