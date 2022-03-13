use std::f64::consts::FRAC_PI_2;
use crate::Arr4;
use crate::math::{Arr2, Arr3, Vec3, to_nom_vec3, Vec4};

const BORDER_SIZE: f64 = 0.03;

#[derive(Debug)]
pub struct Cam {
    /// current position
    pub pos: Vec4<f64>,
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
    c >= 2
}

pub fn render(scr: &mut Arr2<u8>, cam: &Cam, arr4: &Arr4) {
    //! generate current view by mutating scr
    //! assume distance between eye and screen is 1

    // distance between the center of the highest and the lowest pixel
    // => distance between top and bottom ray
    let h = (scr.y - 1) as f64;
    // between left and right ray
    let w = (scr.x - 1) as f64;

    // normalised vector towards center of the screen (i.e. view angle)
    let tn = to_nom_vec3(cam.theta, cam.phi);
    // normalised vector towards the top of the camera (relative to view angle)
    // assume the top is above (absolute) the camera
    // (will change later)
    let vn = to_nom_vec3(cam.theta, cam.phi - FRAC_PI_2);
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

    let pos = &cam.pos;

    for y in 0..scr.y {
        // copy vector, used for shifting
        let mut ray = Vec3 { ..py };

        for x in 0..scr.x {
            let mut map = Vec3 {
                x: pos.x.floor(),
                y: pos.y.floor(),
                z: pos.x.floor(),
            };
            let delta = Vec3 {
                x: if ray.x == 0.0 { 1e30 } else { 1.0 / ray.x.abs() },
                y: if ray.y == 0.0 { 1e30 } else { 1.0 / ray.y.abs() },
                z: if ray.z == 0.0 { 1e30 } else { 1.0 / ray.z.abs() },
            };
            let step = Vec3 {
                x: if ray.x < 0.0 { -1.0 } else { 1.0 },
                y: if ray.y < 0.0 { -1.0 } else { 1.0 },
                z: if ray.z < 0.0 { -1.0 } else { 1.0 },
            };
            let mut side = Vec3 {
                x: (pos.x - map.x).abs() * delta.x,
                y: (pos.y - map.y).abs() * delta.y,
                z: (pos.z - map.z).abs() * delta.z,
            };

            scr[y][x] = b' ';
            while !is_outside(&map, arr4.x, arr4.y, arr4.z) {
                if (arr4.at(map.x as usize, map.y as usize, map.z as usize, pos.w as usize) & 1) == 1 {
                    scr[y][x] = if is_border(&side) { b'.' } else { b'@' };
                    break;
                }
                if side.x < side.y.min(side.z) {
                    side.x += delta.x;
                    map.x += step.x;
                } else if side.y < side.x.min(side.z) {
                    side.y += delta.y;
                    map.y += step.y;
                } else {
                    side.z += delta.z;
                    map.z += step.z;
                }
            }

            ray += qx;
        }
        py += qy;
    }
}
