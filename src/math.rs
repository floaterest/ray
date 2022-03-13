use std::ops::{Add, AddAssign, Index, IndexMut, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

#[derive(Debug, Clone, Copy)]
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
}

pub fn to_nom_vec3(theta: f64, phi: f64) -> Vec3<f64> {
    //! convert (1, θ, φ) to (x, y, z) (normalised)
    //! uses the *mathematics* notation, i.e. azimuthal angle θ, polar angle φ
    //! https://en.wikipedia.org/wiki/Spherical_coordinate_system
    Vec3 {
        x: phi.sin() * theta.cos(),
        y: phi.sin() * theta.sin(),
        z: phi.cos(),
    }
}

//#region init
impl<T: Clone> Arr3<T> {
    pub fn new(x: usize, y: usize, z: usize, fill: T) -> Arr3<T> {
        Arr3 {
            items: vec![vec![vec![fill; x]; y]; z],
            x,
            y,
            z,
        }
    }
}

impl<T: Clone> Arr2<T> {
    pub fn new(x: usize, y: usize, fill: T) -> Arr2<T> {
        Arr2 {
            items: vec![vec![fill; x]; y],
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
impl<T: Add<Output=T>> Add for Vec3<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: AddAssign> AddAssign for Vec3<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T: Sub<Output=T>> Sub for Vec3<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

/// scaling
impl<T: Mul<Output=T>> Mul<T> for Vec3<T> where T: Copy {
    type Output = Self;
    fn mul(self, scale: T) -> Self {
        Self {
            x: self.x * scale,
            y: self.y * scale,
            z: self.z * scale,
        }
    }
}

/// cross product
impl<T: Mul<Output=T> + Sub<Output=T>> Mul for Vec3<T> where T: Copy {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}
//#endregion vec operations