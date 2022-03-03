use std::ops::{Add, Index, IndexMut, Mul, Sub};

#[derive(Debug)]
pub struct Vec3<T> {
    x: T,
    y: T,
    z: T,
}

pub struct Ang3 {
    // https://en.wikipedia.org/wiki/Spherical_coordinate_system
    theta: f64,
    phi: f64,
}

#[derive(Debug)]
pub struct Arr3<T> {
    items: Vec<Vec<Vec<T>>>,
    dim: Vec3<usize>,
}

impl Vec3<f64> {
    pub fn normalise(&self) -> Vec3<f64> {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Vec3 {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }
}

impl<T: Clone> Arr3<T> {
    pub fn new(x: usize, y: usize, z: usize, fill: T) -> Arr3<T> {
        Arr3 {
            items: vec![vec![vec![fill; z]; y]; x],
            dim: Vec3 { x, y, z },
        }
    }
}

impl<T> Index<usize> for Arr3<T> {
    type Output = Vec<Vec<T>>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}

impl<T> IndexMut<usize> for Arr3<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.items[index]
    }
}

//#region vector addition
impl Add for Vec3<f64> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3<f64> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
//#endregion vector addition

impl Mul<f64> for Vec3<f64> {
    type Output = Self;
    fn mul(self, scale: f64) -> Self {
        Self {
            x: self.x * scale,
            y: self.y * scale,
            z: self.z * scale,
        }
    }
}