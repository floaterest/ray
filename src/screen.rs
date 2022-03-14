use std::cmp::Ordering;
use std::f64::consts::FRAC_PI_2;
use std::ops::{Index, IndexMut};
use crate::{Cam, Map};
use crate::math::{to_vec3n, Vec3};

const BORDER_SIZE: f64 = 0.03;


#[derive(Debug)]
pub struct Screen {
    pub x: usize,
    pub y: usize,

    content: Vec<Vec<u8>>,
}

//#region index
impl Index<usize> for Screen {
    type Output = Vec<u8>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.content[index]
    }
}

impl IndexMut<usize> for Screen {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.content[index]
    }
}
//#endregion index

fn is_border(pos: &Vec3<f64>) -> bool {
    //! return whether pos it near a border (i.e. grid line)
    [pos.x, pos.y, pos.z].iter().filter(|&&n| (n - n.round()).abs() < BORDER_SIZE).count() >= 2
}

impl Screen {
    pub fn new(x: usize, y: usize, fill: u8) -> Screen {
        Screen {
            content: vec![vec![fill; x]; y],
            x,
            y,
        }
    }

    pub fn render(&mut self, cam: &Cam, map: &Map) {
        //! render current view, assume distance between eye and screen is 1

        // clear screen
        self.content = vec![vec![b' '; self.x]; self.y];
        // dist(top_ray, bottom_ray) = dist(highest_px, lowest_px) = self.y - 1
        let w = (self.x - 1) as f64;
        let h = (self.y - 1) as f64;

        // tn: eye -> center of screen
        let tn = to_vec3n(cam.theta, cam.phi);
        // vn: eye -> top (will change later)
        let vn = to_vec3n(cam.theta, cam.phi - FRAC_PI_2);
        // bn: eye -> left (using right hand rule)
        let bn = vn * tn;

        // dim of screen halved
        let gx = cam.fov2.tan();
        let gy = gx * h / w;

        // py: eye to screen at (0,y) for 0 <= y < self.y
        let mut py = tn + bn * gx + vn * gy;
        // used to shift to the next pixel
        let qx = bn * (-2.0 * gx / w);
        let qy = vn * (-2.0 * gy / h);

        let pos = &cam.pos;
        for y in 0..self.y {
            let mut ray = Vec3 { ..py };
            for x in 0..self.x {
                let delta = Vec3::compose(|i| if ray[i] == 0.0 { 1e30 } else { 1.0 / ray[i].abs() });
                let step = Vec3::compose(|i| if ray[i] < 0.0 { -1.0 } else { 1.0 });
                let mut block = Vec3::compose(|i| pos[i].floor());
                let mut side = Vec3::compose(|i| (pos[i] - block[i]).abs() * delta[i]);
                let mut dist = 0.0;
                while map.is_inside(block.x, block.y, block.z, pos.w) {
                    let hit = map.index(
                        block.x as usize,
                        block.y as usize,
                        block.z as usize,
                        pos.w as usize,
                    );
                    if (hit & 1) == 1 {
                        // is opaque
                        let p = Vec3::compose(|i| pos[i] + ray[i] * dist);
                        self[y][x] = if is_border(&p) { b'.' } else { b'@' };
                        break;
                    }

                    if side.x < side.y.min(side.z) {
                        dist = side.x;
                        side.x += delta.x;
                        block.x += step.x;
                    } else if side.y < side.x.min(side.z) {
                        dist = side.y;
                        side.y += delta.y;
                        block.y += step.y;
                    } else {
                        dist = side.z;
                        side.z += delta.z;
                        block.z += step.z;
                    }
                }
                ray += qx;
            }
            py += qy;
        }
    }
}
