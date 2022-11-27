use std::ops::{Add, AddAssign, Index, IndexMut, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

/*
(might use spherical coordinates later)
pub fn to_vec3n(theta: f64, phi: f64) -> Vec3<f64> {
    //! convert (1, θ, φ) to (x, y, z) normalised
    //! uses the *mathematics* notation, i.e. azimuthal angle θ, polar angle φ
    //! https://en.wikipedia.org/wiki/Spherical_coordinate_system
    Vec3 {
        x: phi.sin() * theta.cos(),
        y: phi.sin() * theta.sin(),
        z: phi.cos(),
    }
}
*/

//#region impl Vec3<T>
impl<T> Vec3<T> {
    // too lazy to type xyz
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl Vec3<f64> {
    #[allow(dead_code)] // will use later
    pub fn rotate(&mut self, &axis: &Vec3<f64>, angle: f64) {
        //! rotate angle along axis
        *self = *self * angle.cos() + (axis * *self) * angle.sin() + axis * self.dot(&axis) * (1.0 - angle.cos());
    }
}

impl<T: Copy> Vec3<T> {
    pub fn compose<F: Fn(usize) -> T>(f: F) -> Self {
        //! apply the same transformation to each value
        Self { x: f(0), y: f(1), z: f(2) }
    }
}

// dot product
impl<T: Copy + Mul<Output=T> + Add<Output=T>> Vec3<T> {
    pub fn dot(&self, &rhs: &Self) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

//#region Neg, Add, Sub, AddAssign, Mul
impl<T: Copy + Neg<Output=T>> Neg for Vec3<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec3::compose(|i| -self[i])
    }
}

impl<T: Copy + Add<Output=T>> Add for Vec3<T> {
    type Output = Vec3<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3::compose(|i| self[i] + rhs[i])
    }
}

impl<T: Copy + Sub<Output=T>> Sub for Vec3<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Vec3::compose(|i| self[i] - rhs[i])
    }
}

impl<T: Copy + AddAssign> AddAssign for Vec3<T> {
    fn add_assign(&mut self, rhs: Self) {
        (0..3).for_each(|i| self[i] += rhs[i]);
    }
}

// scalar multiplication
impl<T: Copy + Mul<Output=T>> Mul<T> for Vec3<T> {
    type Output = Self;
    fn mul(self, scale: T) -> Self {
        Vec3::compose(|i| self[i] * scale)
    }
}

/// cross product
impl<T: Copy + Mul<Output=T> + Sub<Output=T>> Mul for Vec3<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

//#endregion Neg, Add, Sub, AddAssign, Mul

//#region Index, IndexMut
impl<T> Index<usize> for Vec3<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            i => panic!("{} is not a valid index for 3d vec, must be in 0..3", i),
        }
    }
}

impl<T> IndexMut<usize> for Vec3<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            i => panic!("{} is not a valid index for 3d vec, must be in 0..3", i),
        }
    }
}
//#endregion Index, IndexMut

//#endregion impl Vec3<T>