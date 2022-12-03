use std::cmp::Ordering;

use crate::{Camera, Map, Vec3};

const BORDER_SIZE: f64 = 0.03;
const AIR: u8 = 0;
const OPAQUE: u8 = 127;
const BORDER: u8 = 255;

fn is_border(pos: &Vec3<f64>) -> bool {
    //! return whether pos it near block border

    // if 2 of (x,y,z) is close to an integer <=> close to border
    [pos.x, pos.y, pos.z].iter().filter(|&&n| (n - n.round()).abs() < BORDER_SIZE).count() >= 2
}

pub struct Trace<'a> {
    map: &'a Map,
    cam: &'a Camera,
    ray: Vec3<f64>,
    /// delta.x = distance from (x1, y1, z1) to (x1+1, y2, z2) along the ray
    delta: Vec3<f64>,
    /// step.x = shift to the next block on x-axis
    step: Vec3<f64>,
    /// the block that the ray is at (starts at camera)
    block: Vec3<f64>,
    /// side.x = distance between pov.x and a side of block on x axis
    side: Vec3<f64>,
    norm: f64,
}

impl<'a> Trace<'a> {
    /// `ray` is unit vector
    pub fn new(map: &'a Map, cam: &'a Camera, ray: Vec3<f64>) -> Self {
        let delta = Vec3::compose(|i| 1.0 / ray[i].abs());
        let block = Vec3::compose(|i| cam.pos[i].floor());
        Self {
            map,
            cam,
            ray,
            delta,
            block,
            step: Vec3::compose(|i| if ray[i] < 0.0 { -1.0 } else { 1.0 }),
            side: Vec3::compose(|i| (cam.pos[i] - block[i]).abs() * delta[i]),
            norm: 0.0,
        }
    }
}

impl<'a> Iterator for Trace<'a> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        let (map, cam) = &(self.map, self.cam);
        let (ray, delta, step) = &(self.ray, self.delta, self.step);
        let (block, side) = &(self.block, self.side);

        if map.outside(&block) { return None; }
        let hit = *map.at(&Vec3::compose(|i| block[i] as usize));
        // if hit opaque
        if hit > 0 {
            let target = Vec3::compose(|i| cam.pos[i] + ray[i] * self.norm);
            self.block = Vec3::compose(|_| -1.0);
            Some(if is_border(&target) { OPAQUE } else { BORDER })
        } else {
            let min = (0..3).min_by(|&i, &j| side[i].partial_cmp(&side[j]).unwrap_or(Ordering::Equal)).unwrap();
            // go to nearest border
            self.norm = side[min];
            self.side[min] += delta[min];
            self.block[min] += step[min];
            Some(AIR)
        }
    }
}
