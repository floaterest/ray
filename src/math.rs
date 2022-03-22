use std::ops::{Add, AddAssign, Index, IndexMut, Mul, Neg, Sub};

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
/* (might use spherical coordinates later)
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
impl<T> Vec3<T> where T: Copy {
    pub fn compose<F: Fn(u8) -> T>(f: F) -> Self {
        Self {
            x: f(1),
            y: f(2),
            z: f(3),
        }
    }
}

impl<T> Vec3<T> where T: Copy + Mul<Output=T> + Add<Output=T> {
    pub fn dot(&self, &rhs: &Self) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Vec3<f64> {
    pub fn rotate(&mut self, &axis: &Vec3<f64>, angle: f64) {
        *self = *self * angle.cos() + (axis * *self) * angle.sin() + axis * self.dot(&axis) * (1.0 - angle.cos());
    }
}

impl<T> Index<u8> for Vec3<T> {
    type Output = T;
    fn index(&self, index: u8) -> &Self::Output {
        match index {
            1 => &self.x,
            2 => &self.y,
            3 => &self.z,
            i => panic!("{} is not a valid index for 3d vec, must be in [1,3]", i),
        }
    }
}

impl<T> IndexMut<u8> for Vec3<T> {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        match index {
            1 => &mut self.x,
            2 => &mut self.y,
            3 => &mut self.z,
            i => panic!("{} is not a valid index for 3d vec, must be in [1,3]", i),
        }
    }
}

impl<T: Neg<Output=T>> Neg for Vec3<T> where T: Copy {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3::compose(|i| -self[i])
    }
}

impl<T: Add<Output=T>> Add for Vec3<T> where T: Copy {
    type Output = Vec3<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3::compose(|i| self[i] + rhs[i])
    }
}

impl<T: AddAssign> AddAssign for Vec3<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T: Sub<Output=T>> Sub for Vec3<T> where T: Copy {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Vec3::compose(|i| self[i] - rhs[i])
    }
}

/// scaling
impl<T: Mul<Output=T>> Mul<T> for Vec3<T> where T: Copy {
    type Output = Self;
    fn mul(self, scale: T) -> Self {
        Vec3::compose(|i| self[i] * scale)
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
//#endregion impl Vec3<T>

//#region impl Vec4<T>
impl<T> Index<u8> for Vec4<T> {
    type Output = T;
    fn index(&self, index: u8) -> &Self::Output {
        match index {
            1 => &self.x,
            2 => &self.y,
            3 => &self.z,
            4 => &self.w,
            i => panic!("{} is not a valid index for 4d vec, must be in [1,4]", i),
        }
    }
}

impl<T> IndexMut<u8> for Vec4<T> {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        match index {
            1 => &mut self.x,
            2 => &mut self.y,
            3 => &mut self.z,
            4 => &mut self.w,
            i => panic!("{} is not a valid index for 3d vec, must be in [1,4]", i),
        }
    }
}

//#endregion impl Vec4<T>