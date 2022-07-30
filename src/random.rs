use rand::prelude::*;

#[inline(always)]
pub fn random_f64() -> f64 {
    let mut rng = thread_rng();
    rng.gen_range(0.0..1.0)
}

#[inline(always)]
pub fn random_f64_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_f64()
}
