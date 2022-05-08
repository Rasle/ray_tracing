use std::rc::Rc;

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::material::Material;

pub struct HitRecord {
	pub p : Vec3,
	pub normal : Vec3,
	pub mat : Rc<dyn Material>,
	pub t : f64,
	pub front_facing : bool,
}

impl HitRecord {
	fn set_face_normal(&mut self, r : Ray, outward_normal : Vec3) {
		self.front_facing = Vec3::dot(r.direction, outward_normal) < 0.0;
		self.normal = if self.front_facing {outward_normal} else {-outward_normal};
	}
}

pub trait Hittable {
	fn hit(&self, r : Ray, t_min : f64, t_max : f64) -> Option<HitRecord>;
}

pub struct HittableList {
    pub objects : Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects : Vec::new() }
    }

    // pub fn new_add(object : Box<dyn Hittable>) -> HittableList {
    //     HittableList { objects : vec![object] }
    // }

    pub fn add(&mut self, object : Box<dyn Hittable>) {
        self.objects.push(object);
    }

    // pub fn clear(&mut self) {
    //     self.objects.clear();
    // }
}

impl Hittable for HittableList {
	fn hit(&self, r : Ray, t_min : f64, t_max : f64) -> Option<HitRecord> {
		let mut hit = None;
		let mut closest_so_far = t_max;
		for hittable in self.objects.iter() {
			if let Some(candidate_hit) = hittable.hit(r, t_min, closest_so_far) {
				closest_so_far = candidate_hit.t;
				hit = Some(candidate_hit);
			}
		}

		hit
	}
}

pub struct Sphere {
	pub center : Vec3,
	pub radius : f64,
	pub material : Rc<dyn Material>,
}

impl Sphere {
	pub fn new(center : Vec3, radius : f64, material : Rc<dyn Material>) -> Self {
		Sphere { center, radius, material }
	}
}

impl Hittable for Sphere {
	fn hit(&self, r : Ray, t_min : f64, t_max : f64) -> Option<HitRecord> {
		let oc = r.origin - self.center;
		let a = r.direction.length_squared();
		let half_b = Vec3::dot(oc, r.direction);
		let c = oc.length_squared() - self.radius * self.radius;

		let discriminant  = half_b * half_b - a * c;
		if discriminant < 0.0 {
			return None
		}

		let sqrt_discriminant = discriminant.sqrt();
		let mut root = (-half_b - sqrt_discriminant) / a;
		if root < t_min || t_max < root {
            root = (-half_b + sqrt_discriminant) / a;
            if root < t_min || t_max < root {
                return None
            }
		}

		let p = r.at(root);
		let outward_normal = (p - self.center) / self.radius;
		let mut hit_record = HitRecord {
			p,
			t : root,
			normal : outward_normal,
			mat : self.material.clone(),
			front_facing : false,
		};
		hit_record.set_face_normal(r, outward_normal);

		Some(hit_record)
	}
}