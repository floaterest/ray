use std::ops::{Add, Index, IndexMut, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(Debug)]
pub struct Arr3<T> {
    items: Vec<Vec<Vec<T>>>,
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

#[derive(Debug)]
pub struct Arr2<T> {
    items: Vec<Vec<T>>,
    pub x: usize,
    pub y: usize,
    pub ratio: f64,
}

pub fn to_unit(vec3: Vec3<f64>) -> Vec3<f64> {
    let len = (vec3.x * vec3.x + vec3.y * vec3.y + vec3.z * vec3.z).sqrt();
    Vec3 {
        x: vec3.x / len,
        y: vec3.y / len,
        z: vec3.z / len,
    }
}

pub fn to_vec3(theta: f64, phi: f64) -> Vec3<f64> {
    Vec3 {
        x: phi.sin() * theta.cos(),
        y: phi.sin() * theta.sin(),
        z: phi.cos(),
    }
}

//#region impl
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
            ratio: (y as f64) / (x as f64),
        }
    }
}
//#endregion impl

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
    type Output = Vec<T>;
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
impl Neg for Vec3<f64> {
    type Output = Self;
    fn neg(self) -> Self {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vec3<f64> {
    type Output = Vec3<f64>;
    fn add(self, other: Self) -> Vec3<f64> {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3<f64> {
    type Output = Vec3<f64>;
    fn sub(self, other: Self) -> Vec3<f64> {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f64> for Vec3<f64> {
    type Output = Self;
    fn mul(self, scale: f64) -> Vec3<f64> {
        Vec3 {
            x: self.x * scale,
            y: self.y * scale,
            z: self.z * scale,
        }
    }
}

/// cross product
impl Mul for Vec3<f64> {
    type Output = Vec3<f64>;
    fn mul(self, other: Self) -> Vec3<f64> {
        Vec3 {
            x: self.y * other.z - self.z - other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}
//#endregion vec operations