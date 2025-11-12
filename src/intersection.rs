use crate::utils::Vec3;
use crate::material::Material;

#[derive(Clone)]
pub struct Intersection {
    pub t: f32,
    pub position: Vec3,
    pub normal: Vec3,
    pub material: Material,
    pub u: f32,
    pub v: f32,
}

impl Intersection {
    pub fn new(t: f32, position: Vec3, normal: Vec3, material: Material, u: f32, v: f32) -> Self {
        Self {
            t,
            position,
            normal,
            material,
            u,
            v,
        }
    }
}
