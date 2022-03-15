use crate::{Vec3, Vec4};

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

fn rotate(&src: &Vec3<f64>, &axis: &Vec3<f64>, cos: f64, sin: f64) -> Vec3<f64> {
    src * cos + (axis * src) * sin + axis * axis.dot(&src) * (1.0 - cos)
}

impl Cam {
    pub fn rotate_left(&mut self, cos: f64, sin: f64) {
        self.pov = rotate(&self.pov, &self.top, cos, sin);
    }
    pub fn rotate_top(&mut self, cos: f64, sin: f64) {
        let axis = self.pov * self.top;
        self.pov = rotate(&self.pov, &axis, cos, sin);
        self.top = rotate(&self.top, &axis, cos, sin);
    }
}