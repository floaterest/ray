use crate::Vec3;

pub struct Map {
    pub size: Vec3<usize>,
    pub data: Vec<Vec<Vec<bool>>>,
}

impl Map {
    pub fn at(&self, x: usize, y: usize, z: usize) -> &bool {
        &self.data[z][y][x]
    }
    pub fn inside(&self, x: f64, y: f64, z: f64) -> bool {
        x >= 0.0 && y >= 0.0 && z >= 0.0
            && x < self.size.x as f64 && y < self.size.y as f64 && z < self.size.z as f64
    }
}
