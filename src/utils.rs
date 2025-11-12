use std::ops::{Add, Sub, Mul, Div, Neg};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self { Self { x, y, z } }
    pub fn zero() -> Self { Self::new(0.0, 0.0, 0.0) }
    pub fn one() -> Self { Self::new(1.0, 1.0, 1.0) }

    pub fn dot(&self, other: &Vec3) -> f32 { self.x * other.x + self.y * other.y + self.z * other.z }
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn length(&self) -> f32 { (self.x * self.x + self.y * self.y + self.z * self.z).sqrt() }

    pub fn normalize(&self) -> Vec3 {
        let len = self.length();
        if len > 0.0 { *self / len } else { *self }
    }

    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        *self - *normal * 2.0 * self.dot(normal)
    }

    pub fn refract(&self, normal: &Vec3, eta: f32) -> Option<Vec3> {
        let cos_i = -self.dot(normal).max(-1.0).min(1.0);
        let sin_t2 = eta * eta * (1.0 - cos_i * cos_i);
        if sin_t2 > 1.0 { None } else {
            let cos_t = (1.0 - sin_t2).sqrt();
            Some(*self * eta + *normal * (eta * cos_i - cos_t))
        }
    }
}

impl Add for Vec3 { type Output = Vec3; fn add(self, o: Vec3) -> Vec3 { Vec3::new(self.x + o.x, self.y + o.y, self.z + o.z) } }
impl Sub for Vec3 { type Output = Vec3; fn sub(self, o: Vec3) -> Vec3 { Vec3::new(self.x - o.x, self.y - o.y, self.z - o.z) } }
impl Mul<f32> for Vec3 { type Output = Vec3; fn mul(self, s: f32) -> Vec3 { Vec3::new(self.x * s, self.y * s, self.z * s) } }
impl Div<f32> for Vec3 { type Output = Vec3; fn div(self, s: f32) -> Vec3 { Vec3::new(self.x / s, self.y / s, self.z / s) } }
impl Neg for Vec3 { type Output = Vec3; fn neg(self) -> Vec3 { Vec3::new(-self.x, -self.y, -self.z) } }

pub fn lerp(a: f32, b: f32, t: f32) -> f32 { a + (b - a) * t }
pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    if value < min { min } else if value > max { max } else { value }
}
