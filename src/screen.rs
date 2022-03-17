use std::cmp::Ordering;
use std::ops::{Index, IndexMut};

use crate::{Cam, Map};
use crate::math::Vec3;

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

    // if 2 of (x,y,z) is close to an integer => close to a grid line
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

        // top x front = left (right hand rule)
        let tn = cam.pov;
        let vn = cam.top;
        let bn = vn * tn;

        // dim of screen halved
        let gx = cam.fov2.tan();
        let gy = gx * h / w;

        // py: eye to screen at (0,y) for 0 <= y < self.y
        let mut py = tn + bn * gx + vn * gy;
        // used to shift to the next pixel
        let qx = bn * (-2.0 * gx / w);
        let qy = vn * (-2.0 * gy / h);

        for y in 0..self.y {
            let mut ray = Vec3 { ..py };
            for x in 0..self.x {
                // delta.x = distance from (1, y1, z1) to (2, y2, z2) following the ray
                let delta = Vec3::compose(|i| if ray[i] == 0.0 { 1e30 } else { 1.0 / ray[i].abs() });
                // step.x = shift to the next block on x axis
                let step = Vec3::compose(|i| if ray[i] < 0.0 { -1.0 } else { 1.0 });
                // block = the block that the ray is at
                let mut block = Vec3::compose(|i| cam.pos[i].floor());
                // side.x = distance between pov and a side of a block on x axis
                let mut side = Vec3::compose(|i| (cam.pos[i] - block[i]).abs() * delta[i]);
                // norm of current ray
                let mut dist = 0.0;
                while map.is_inside(block.x, block.y, block.z, cam.pos.w) {
                    // neglect w coordinate
                    let hit = map.index(
                        block.x as usize,
                        block.y as usize,
                        block.z as usize,
                        cam.pos.w as usize,
                    );
                    // if is visible
                    if (hit & 1) == 1 {
                        let target = Vec3::compose(|i| cam.pos[i] + ray[i] * dist);
                        self[y][x] = if is_border(&target) { b'.' } else { b'@' };
                        break;
                    }

                    // shift closest
                    let min = (1..=3).min_by(|&a, &b| side[a].partial_cmp(&side[b]).unwrap_or(Ordering::Equal)).unwrap();
                    dist = side[min];
                    side[min] += delta[min];
                    block[min] += step[min];
                }
                ray += qx;
            }
            py += qy;
        }
    }
}
