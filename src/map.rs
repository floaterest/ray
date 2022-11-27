use crate::Vec3;

pub struct Map {
    pub size: Vec3<usize>,
    // `data[z][y][x] == true iff block at (x,y,z) is opaque`
    data: Vec<Vec<Vec<bool>>>,
}

impl Map {
    pub fn new(size: Vec3<usize>, data: Vec<Vec<Vec<bool>>>) -> Self {
        Self { size, data }
    }
    pub fn at(&self, x: usize, y: usize, z: usize) -> &bool {
        &self.data[z][y][x]
    }
    pub fn inside(&self, x: f64, y: f64, z: f64) -> bool {
        //! check if (x,y,z) is inside the map
        x >= 0.0 && y >= 0.0 && z >= 0.0
            && x < self.size.x as f64 && y < self.size.y as f64 && z < self.size.z as f64
    }
}
