use std::ops;

use crate::random::*;

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
	pub x : f64,
	pub y : f64,
	pub z : f64,
}

impl Vec3 {
	pub fn zeros() -> Vec3 {
		Vec3 {x : 0.0, y : 0.0, z : 0.0}
	}

	pub fn ones() -> Vec3 {
		Vec3 {x : 1.0, y : 1.0, z : 1.0}
	}

	pub fn new(x : f64, y : f64, z : f64) -> Vec3 {
		Vec3 {x, y, z}
	}

	pub fn unit_vector(v : Vec3) -> Vec3 {
		v / v.length()
	}
	
	pub fn length(&self) -> f64 {
		self.length_squared().sqrt()
	}

	pub fn length_squared(&self) -> f64 {
		self.x * self.x + self.y * self.y + self.z * self.z
	}

	pub fn dot(u : Vec3, v : Vec3) -> f64 {
		u.x * v.x +
		u.y * v.y +
		u.z * v.z
	}

    pub fn cross(u : Vec3, v : Vec3) -> Vec3 {
        Vec3 {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x
        }
    }

	pub fn random() -> Vec3 {
		Vec3 {x: random_f64(), y: random_f64(), z: random_f64()}
	}

	pub fn random_range(min : f64, max : f64) -> Vec3 {
		Vec3 {x: random_f64_range(min, max), y: random_f64_range(min, max), z: random_f64_range(min, max)}
	}

	pub fn random_in_unit_sphere() ->  Vec3 {
		loop {
			let p = Vec3::random_range(-1.0, 1.0);
			if p.length_squared() >= 1.0 {
				continue
			}
			break p;
		}
	}

	pub fn random_unit_vector() -> Vec3 {
		Vec3::unit_vector(Vec3::random_in_unit_sphere())
	}

	pub fn random_in_hemisphere(normal : &Vec3) -> Vec3 {
		let in_unit_sphere = Vec3::random_in_unit_sphere();
		if Vec3::dot(in_unit_sphere, *normal) > 0.00 {
			in_unit_sphere
		}
		else {
			-in_unit_sphere
		}
	}

    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::new(random_f64_range(-1.0, 1.0), random_f64_range(-1.0, 1.0), 0.0);
            if p.length_squared() >= 1.0 {
                continue
            }
            break p;
        }
    }

	pub fn near_zero(&self) -> bool {
        const S : f64 = 1.0e-8;
        (self.x.abs() < S) && (self.y.abs() < S) && (self.z.abs() < S)
	}

    pub fn reflect(v : Vec3, n : Vec3) -> Vec3 {
        v - 2.0 * Vec3::dot(v, n) * n
    }

    pub fn refract(uv : Vec3, n : Vec3, etai_over_etat : f64) -> Vec3 {
        let cos_theta = Vec3::dot(-uv, n).min(1.0);
        let r_out_perp = etai_over_etat * (uv + cos_theta * n);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
        r_out_perp + r_out_parallel
    }
}

impl ops::Add<Vec3> for Vec3 {
	type Output = Vec3;

	fn add(self, other : Vec3) -> Vec3 {
		let x = self.x + other.x;
		let y = self.y + other.y;
		let z = self.z + other.z;
		Vec3::new(x, y, z)
	}
}

impl ops::Sub<Vec3> for Vec3 {
	type Output = Vec3;

	fn sub(self, other : Vec3) -> Vec3 {
		let x = self.x - other.x;
		let y = self.y - other.y;
		let z = self.z - other.z;
		Vec3::new(x, y, z)
	}
}

impl ops::Mul<Vec3> for Vec3 {
	type Output = Vec3;

	fn mul(self, other : Vec3) -> Vec3 {
		let x = self.x * other.x;
		let y = self.y * other.y;
		let z = self.z * other.z;
		Vec3::new(x, y, z)
	}
}

impl ops::Div<Vec3> for Vec3 {
	type Output = Vec3;

	fn div(self, other : Vec3) -> Vec3 {
		let x = self.x / other.x;
		let y = self.y / other.y;
		let z = self.z / other.z;
		Vec3::new(x, y, z)
	}
}

impl ops::Add<Vec3> for f64 {
	type Output = Vec3;

	fn add(self, other : Vec3) -> Vec3 {
		let x = self + other.x;
		let y = self + other.y;
		let z = self + other.z;
		Vec3::new(x, y, z)
	}
}

impl ops::Mul<Vec3> for f64 {
	type Output = Vec3;

	fn mul(self, other : Vec3) -> Vec3 {
		let x = self * other.x;
		let y = self * other.y;
		let z = self * other.z;
		Vec3::new(x, y, z)
	}
}

impl ops::Div<Vec3> for f64 {
	type Output = Vec3;

	fn div(self, other : Vec3) -> Vec3 {
		let x = self / other.x;
		let y = self / other.y;
		let z = self / other.z;
		Vec3::new(x, y, z)
	}
}

impl ops::Div<f64> for Vec3 {
	type Output = Vec3;

	fn div(self, t : f64) -> Vec3 {
		(1.0 / t) * self
	}
}

impl ops::Neg for Vec3 {
	type Output = Vec3;

	fn neg(self) -> Vec3 {
		Vec3::new(-self.x, -self.y, -self.z)
	}
}

impl ops::AddAssign for Vec3 {
	fn add_assign(&mut self, rhs: Self) {
		self.x += rhs.x;
		self.y += rhs.y;
		self.z += rhs.z;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_add() {
		let v1 = Vec3::new(1.0, 1.0, 1.0);
		let v2 = Vec3::new(1.0, 1.0, 1.0);

		let v3 = v1 + v2;

		assert_eq!(2.0, v3.x);
		assert_eq!(2.0, v3.y);
		assert_eq!(2.0, v3.z);
	}

	#[test]
	fn test_add_constant() {
		let v1 = Vec3::new(1.0, 1.0, 1.0);

		let v3 = 1.0 + v1;

		assert_eq!(2.0, v3.x);
		assert_eq!(2.0, v3.y);
		assert_eq!(2.0, v3.z);
	}

	#[test]
	fn test_sub() {
		let v1 = Vec3::new(1.0, 1.0, 1.0);
		let v2 = Vec3::new(1.0, 1.0, 1.0);

		let v3 = v1 - v2;

		assert_eq!(0.0, v3.x);
		assert_eq!(0.0, v3.y);
		assert_eq!(0.0, v3.z);
	}

	#[test]
	fn test_mul() {
		let v1 = Vec3::new(2.0, 2.0, 2.0);
		let v2 = Vec3::new(2.0, 2.0, 2.0);

		let v3 = v1 * v2;

		assert_eq!(4.0, v3.x);
		assert_eq!(4.0, v3.y);
		assert_eq!(4.0, v3.z);
	}

	#[test]
	fn test_mul_constant() {
		let v1 = Vec3::new(2.0, 2.0, 2.0);

		let v3 = 2.0 * v1;

		assert_eq!(4.0, v3.x);
		assert_eq!(4.0, v3.y);
		assert_eq!(4.0, v3.z);
	}


	#[test]
	fn test_div() {
		let v1 = Vec3::new(2.0, 2.0, 2.0);
		let v2 = Vec3::new(2.0, 2.0, 2.0);

		let v3 = v1 / v2;

		assert_eq!(1.0, v3.x);
		assert_eq!(1.0, v3.y);
		assert_eq!(1.0, v3.z);
	}

	#[test]
	fn test_div_constant() {
		let v1 = Vec3::new(2.0, 2.0, 2.0);

		let v3 = v1 / 2.0;

		assert_eq!(1.0, v3.x);
		assert_eq!(1.0, v3.y);
		assert_eq!(1.0, v3.z);
	}

	#[test]
	fn test_div_constant2() {
		let v1 = Vec3::new(2.0, 2.0, 2.0);

		let v3 = 2.0 / v1;

		assert_eq!(1.0, v3.x);
		assert_eq!(1.0, v3.y);
		assert_eq!(1.0, v3.z);
	}

	#[test]
	fn test_length() {
		let v1 = Vec3::new(2.0, 2.0, 1.0);

		let length = v1.length();

		assert_eq!(3.0, length);
	}

    #[test]
	fn test_length_squared() {
		let v1 = Vec3::new(2.0, 2.0, 1.0);

		let length = v1.length_squared();

		assert_eq!(9.0, length);
	}
}