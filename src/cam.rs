use crate::{to_vec3n, Vec3, Vec4};

const PHI: Vec3<f64> = Vec3 { x: -1.0, y: 0.0, z: 0.0 };
const THETA: Vec3<f64> = Vec3 { x: 0.0, y: 0.0, z: 1.0 };

#[derive(Debug)]
pub struct Cam {
    /// current position
    pub pos: Vec4<f64>,
    // vec from eye to center of screen
    pub pov: Vec3<f64>,
    // vec from eye to top
    pub top: Vec3<f64>,
    /// fov/2
    pub fov2: f64,
}

fn rotate(&src: &Vec3<f64>, &axis: &Vec3<f64>, angle: f64) -> Vec3<f64> {
    src * angle.cos() + (axis * src) * angle.sin() + axis * axis.dot(&src) * (1.0 - angle.cos())
}

impl Cam {
    pub fn rotate_left(&mut self, angle: f64) {
        self.pov = rotate(&self.pov, &self.top, angle);
    }
    pub fn rotate_down(&mut self, angle: f64) {
        let left = self.top * self.pov;
        self.pov = rotate(&self.pov, &left, angle);
        self.top = rotate(&self.top, &left, angle);
    }
}