use std::ops::{Add, Index, IndexMut, Mul, Sub};

#[derive(Debug)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(Debug)]
pub struct Ang3 {
    // https://en.wikipedia.org/wiki/Spherical_coordinate_system
    pub theta: f64,
    pub phi: f64,
}

#[derive(Debug)]
pub struct Arr3<T> {
    items: Vec<Vec<Vec<T>>>,
    x: usize,
    y: usize,
    z: usize,
}

#[derive(Debug)]
pub struct Arr2<T> {
    items: Vec<Vec<T>>,
    x: usize,
    y: usize,
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

//#region init
impl<T: Clone> Arr3<T> {
    pub fn new(x: usize, y: usize, z: usize, fill: T) -> Arr3<T> {
        Arr3 {
            items: vec![vec![vec![fill; z]; y]; x],
            x,
            y,
            z,
        }
    }
}

impl<T: Clone> Arr2<T> {
    pub fn new(x: usize, y: usize, fill: T) -> Arr2<T> {
        Arr2 {
            items: vec![vec![fill; y]; x],
            x,
            y,
        }
    }
}
//#endregion init

//#region index
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

impl<T> Index<usize> for Arr2<T> {
    type Output = Vec<Vec<T>>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}

impl<T> IndexMut<usize> for Arr2<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.items[index]
    }
}
//#endregion index

//#region vec operations
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

/// scaling
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
//#endregion vec operations