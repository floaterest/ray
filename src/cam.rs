use std::f64::consts::FRAC_PI_2;
use crate::math::{Arr2, Arr3, to_unit, to_vec3, Vec3};

const BORDER_SIZE: f64 = 0.03;

#[derive(Debug)]
pub struct Cam {
    pub pos: Vec3<f64>,
    pub theta: f64,
    pub phi: f64,
    /// fov/2
    pub fov2: f64,
}

fn is_outside(pos: &Vec3<f64>, x: usize, y: usize, z: usize) -> bool {
    if pos.x < 0.0 || pos.y < 0.0 || pos.z < 0.0 {
        return true;
    }
    if pos.x >= x as f64 || pos.y >= y as f64 || pos.z >= z as f64 {
        return true;
    }
    false
}

fn is_border(pos: &Vec3<f64>) -> bool {
    let mut c = 0;
    if (pos.x - pos.x.round()).abs() < BORDER_SIZE {
        c += 1;
    }
    if (pos.y - pos.y.round()).abs() < BORDER_SIZE {
        c += 1;
    }
    if (pos.z - pos.z.round()).abs() < BORDER_SIZE {
        c += 1;
    }

    c >= 2
}

fn ray_trace(mut pos: Vec3<f64>, dir: Vec3<f64>, arr3: &Arr3<bool>) -> u8 {
    let eps = 0.01f64;
    while !is_outside(&pos, arr3.x, arr3.y, arr3.z) {
        let not_air = arr3[pos.x as usize][pos.y as usize][pos.z as usize];
        if not_air {
            return if is_border(&pos) { b'.' } else { b'@' };
        }
        pos += dir * eps;
    }
    b' '
}

impl Cam {
    pub fn picture(self, scr: &mut Arr2<u8>, arr3: &Arr3<bool>) {
        let m1: f64 = (scr.y - 1) as f64;
        let k1: f64 = (scr.x - 1) as f64;

        let tn: Vec3<f64> = to_unit(to_vec3(self.theta, self.phi));
        let vn: Vec3<f64> = to_unit(to_vec3(self.theta, self.phi - FRAC_PI_2));
        let bn: Vec3<f64> = vn * tn;

        let gx: f64 = self.fov2.tan();
        let gy: f64 = gx * m1 / k1;

        let mut py: Vec3<f64> = tn + bn * gx + vn * gy;
        let qx: Vec3<f64> = bn * (-2.0 * gx / k1);
        let qy: Vec3<f64> = vn * (-2.0 * gy / m1);
        for y in 0..scr.y {
            let mut px = Vec3 { ..py };
            for x in 0..scr.x {
                scr[y][x] = ray_trace(self.pos, px, &arr3);
                px += qx;
            }
            py += qy;
        }
    }
}