use std::cmp::Ordering;

use crate::{Camera, Map, Vec3};

const BORDER_SIZE: f64 = 0.03;
const AIR: u8 = 0;
const OPAQUE: u8 = 127;
const BORDER: u8 = 255;

fn is_border(pos: &Vec3<f64>) -> bool {
    //! return whether pos it near a border (i.e. grid line)

    // if 2 of (x,y,z) is close to an integer => close to a grid line
    [pos.x, pos.y, pos.z].iter().filter(|&&n| (n - n.round()).abs() < BORDER_SIZE).count() >= 2
}

pub struct Trace<'a> {
    map: &'a Map,
    cam: &'a Camera,
    ray: Vec3<f64>,
    delta: Vec3<f64>,
    step: Vec3<f64>,
    block: Vec3<f64>,
    side: Vec3<f64>,
    norm: f64, // todo put norm inside ray (as a non-unit vector)?
}

impl<'a> Trace<'a> {
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
        let (block, side, norm) = &(self.block, self.side, self.norm);

        if map.outside(&block) { return None; }
        let hit = *map.at(&Vec3::compose(|i| block[i] as usize));
        // if hit opaque
        if hit > 0 {
            let target = Vec3::compose(|i| cam.pos[i] + ray[i] * *norm);
            self.block = Vec3::compose(|_| -1.0);
            Some(if is_border(&target) { OPAQUE } else { BORDER })
        } else {
            let min = (0..3).min_by(|&i, &j| side[i].partial_cmp(&side[j]).unwrap_or(Ordering::Equal)).unwrap();
            self.norm = side[min];
            self.side[min] += delta[min];
            self.block[min] += step[min];
            Some(AIR)
        }
    }
}
