use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::sphere::HitRecord;
use crate::random::*;

pub trait Material : Sync {
    fn scatter(&self, r_in : &Ray, rec : &HitRecord) -> Option<(Vec3, Ray)>;
}

pub struct Lambertian {
    pub albedo : Vec3
}

impl Lambertian {
    pub fn new(albedo : Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in : &Ray, rec : &HitRecord) -> Option<(Vec3, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        Some((self.albedo, Ray::new(rec.p, scatter_direction)))
    }
}

pub struct Metal {
    pub albedo : Vec3,
    pub fuzz : f64
}

impl Metal {
    pub fn new(albedo : Vec3, f : f64) -> Metal {
        let fuzz = if f < 1.0 { f } else { 1.0 };
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in : &Ray, rec : &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = Vec3::reflect(Vec3::unit_vector(r_in.direction), rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_in_unit_sphere());

        if Vec3::dot(scattered.direction, rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        }
        else {
            None
        }
    }
}

pub struct Dielectric {
    pub index_of_refraction : f64
}

impl Dielectric {
    pub fn new(index_of_refraction : f64) -> Dielectric {
        Dielectric { index_of_refraction }
    }

    pub fn reflectance(cosine : f64, ref_idx : f64) -> f64 {
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in : &Ray, rec : &HitRecord) -> Option<(Vec3, Ray)> {
        let refraction_ratio = if rec.front_facing { 1.0 / self.index_of_refraction } else { self.index_of_refraction };

        let unit_direction = Vec3::unit_vector(r_in.direction);
        let cos_theta = Vec3::dot(-unit_direction, rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction =
            if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > random_f64() {
                Vec3::reflect(unit_direction, rec.normal)
            } else {
                Vec3::refract(unit_direction, rec.normal, refraction_ratio)
            };

        let scattered = Ray::new(rec.p, direction);
        Some((Vec3::ones(), scattered))
    }
}