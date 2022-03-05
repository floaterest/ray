use crate::math::{Ang3, Vec3};

#[derive(Debug)]
pub struct Cam {
    pub pos: Vec3<f64>,
    pub ang: Ang3,
}