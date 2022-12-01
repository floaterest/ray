use std::env;
use std::f64::consts::*;
use std::io::{stdout, Write};

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
    // let args: Vec<String> = env::args().collect();
    // let mut r = Reader::new(File::open(&args[1]).unwrap());
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
        forward: Vec3::normal(1.0, 1.0, 1.0),
        upward: Vec3::normal(0.0, 0.0, 1.0),
        fov2: FRAC_PI_4,
    };
    let frame = render(472, 81, &cam, &map);
    // let frame = render(2, 2, &cam, &map);
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
