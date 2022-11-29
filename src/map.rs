use crate::Vec3;

#[derive(Debug)]
pub struct Map {
    pub size: Vec3<usize>,
    // `data[z][y][x] == true iff block at (x,y,z) is opaque`
    data: Vec<Vec<Vec<u8>>>,
}

impl Map {
    pub fn new(size: Vec3<usize>, data: Vec<Vec<Vec<u8>>>) -> Self {
        Self { size, data }
    }
    pub fn at(&self, coords: &Vec3<usize>) -> &u8 {
        &self.data[coords.z][coords.y][coords.x]
    }
    pub fn outside(&self, v: &Vec3<f64>) -> bool {
        //! check if v is inside the map
        v.x < 0.0 || v.y < 0.0 && v.z < 0.0
            || v.x >= self.size.x as f64 || v.y >= self.size.y as f64 || v.z >= self.size.z as f64
    }
}
