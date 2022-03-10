use std::f64::consts::FRAC_PI_2;
use crate::block::Block;
use crate::math::{Arr2, Arr3, Vec3, to_vec3};

const BORDER_SIZE: f64 = 0.03;
const EPSILON: f64 = 0.01;

#[derive(Debug)]
pub struct Cam {
    /// current position
    pub pos: Vec3<f64>,
    /// azimuthal angle
    pub theta: f64,
    /// polar angle
    pub phi: f64,
    /// fov/2
    pub fov2: f64,
}

fn is_outside(pos: &Vec3<f64>, x: usize, y: usize, z: usize) -> bool {
    //! return whether pos is out of bound
    if pos.x < 0.0 || pos.y < 0.0 || pos.z < 0.0 {
        return true;
    }
    if pos.x >= x as f64 || pos.y >= y as f64 || pos.z >= z as f64 {
        return true;
    }
    false
}

fn is_border(pos: &Vec3<f64>) -> bool {
    //! return whether pos it near a border (i.e. grid line)
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
    let a = 0;
    let b:i32 = 10;
    c >= 2
}

fn ray_trace(mut pos: Vec3<f64>, dir: &Vec3<f64>, arr3: &Arr3<Block>) -> u8 {
    //! shoot a ray and see where it hits
    while !is_outside(&pos, arr3.x, arr3.y, arr3.z) {
        match arr3[pos.z as usize][pos.y as usize][pos.x as usize] {
            Block::Solid => {
                return if is_border(&pos) { b'.' } else { b'@' };
            }
            _ => pos += *dir * EPSILON,
        }
    }
    b' '
}

pub fn render(scr: &mut Arr2<u8>, cam: &Cam, map_data: &Arr3<Block>) {
    //! generate current view by mutating scr
    //! assume distance between eye and screen is 1

    // distance between the center of the highest and the lowest pixel
    // => distance between top and bottom ray
    let h = (scr.y - 1) as f64;
    // between left and right ray
    let w = (scr.x - 1) as f64;

    // normalised vector towards center of the screen (i.e. view angle)
    let tn = to_vec3(cam.theta, cam.phi);
    // normalised vector towards the top of the camera (relative to view angle)
    // assume the top is above (absolute) the camera
    // (will change later)
    let vn = to_vec3(cam.theta, cam.phi - FRAC_PI_2);
    // normalised vector towards the left of the camera
    // (according to the right-hand rule of cross product)
    let bn = vn * tn;

    // length of half of the screen
    let gx = cam.fov2.tan();
    let gy = gx * h / w;

    // ray at (0,0), but will be mutated to (0,y) for 0 <= y < scr.y
    let mut py = tn + bn * gx + vn * gy;
    // next-pixel shifting vector
    let qx = bn * (-2.0 * gx / w);
    let qy = vn * (-2.0 * gy / h);

    for y in 0..scr.y {
        // copy vector, used for shifting
        let mut px = Vec3 { ..py };

        for x in 0..scr.x {
            scr[y][x] = ray_trace(cam.pos, &px, &map_data);
            px += qx;
        }
        py += qy;
    }
}
