use crate::{Map, Vec3, Vec4};

#[derive(Debug)]
pub struct Cam {
    /// current position
    pub pos: Vec4<f64>,
    // vec from eye to center of screen
    pub front: Vec3<f64>,
    // vec from eye to top
    pub down: Vec3<f64>,
    pub right: Vec3<f64>,
    /// fov/2
    pub fov2: f64,
}

impl Cam {
    pub fn pitch(&mut self, angle: f64) {
        //! look up
        self.front.rotate(&self.right, angle);
        self.down.rotate(&self.right, angle);
    }
    pub fn roll(&mut self, angle: f64) {
        //! tilt head right
        self.right.rotate(&self.front, angle);
        self.down.rotate(&self.front, angle);
    }
    pub fn yaw(&mut self, angle: f64) {
        //! turn right
        self.front.rotate(&self.down, angle);
        self.right.rotate(&self.down, angle);
    }

    fn displace(&mut self, new: Vec3<f64>, map: &Map) {
        if map.is_inside(new.x, new.y, new.z, self.pos.w) {
            (1..=3).for_each(|i| self.pos[i] = new[i]);
        }
    }

    pub fn move_forward(&mut self, dist: f64, map: &Map) {
        self.displace(Vec3::compose(|i| self.pos[i] + self.front[i] * dist), map);
    }
    pub fn move_right(&mut self, dist: f64, map: &Map) {
        self.displace(Vec3::compose(|i| self.pos[i] + self.right[i] * dist), map);
    }
}