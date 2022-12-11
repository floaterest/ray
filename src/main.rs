use std::env;
use std::f64::consts::*;
use std::io::{stdout, Write};
use std::sync::Arc;

use crate::{
    camera::Camera,
    map::Map,
    render::render,
    trace::Trace,
    vec3::Vec3,
};

mod vec3;
mod camera;
mod render;
mod map;
mod trace;

fn main() {
    let s = 9;
    let map = Map::new(Vec3::new(s, s, s), (0..s).map(
        |z| (0..s).map(
            |y| (0..s).map(|x| (x % (s - 1), y % (s - 1), z % (s - 1))).map(
                |(x, y, z)| if x * y + y * z + x * z == 0 { 255 } else { 0 }
                // |(x, y, z)| if x * y * z == 0 { 255 } else { 0 }
            ).collect()
        ).collect()
    ).collect());
    let cam = Camera {
        pos: Vec3::new(1.5, 1.5, 1.5),
        forward: Vec3::new(1.0, 1.0, 1.0).normal(),
        upward: Vec3::new(0.0, 0.0, 1.0).normal(),
        fov2: FRAC_PI_4,
    };
    let cam = Arc::new(cam);
    let map = Arc::new(map);
    let frame = render(960, 1080, Arc::clone(&cam), Arc::clone(&map));
    if env::args().len() > 1 { return; }
    let mut w = stdout();
    frame.iter().for_each(|row| {
        w.write_all(&row.iter().map(|n| match n {
            0 => b' ',
            255 => b'.',
            _ => b'#'
        }).collect::<Vec<u8>>()).unwrap();
        w.write_all(&[b'\n']).unwrap();
    })
}
