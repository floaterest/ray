use std::f64::consts::*;
use std::io::{stdout, Write};

use crate::{camera::Camera, map::Map, render::render, vec3::Vec3};

mod vec3;
mod camera;
mod render;
mod map;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let mut r = Reader::new(File::open(&args[1]).unwrap());
    let map = Map::new(Vec3::new(3, 3, 3), vec![vec![
        vec![true, true, true],
        vec![true, true, true],
        vec![true, true, true],
    ], vec![
        vec![true, true, true],
        vec![true, false, true],
        vec![true, true, true],
    ], vec![
        vec![true, true, true],
        vec![true, true, true],
        vec![true, true, true],
    ]]);
    let cam = Camera {
        pos: Vec3::new(1.5, 1.5, 1.5),
        forward: Vec3::new(1.0, 0.0, 0.0),
        upward: Vec3::new(0.0, 0.0, 1.0),
        fov2: FRAC_PI_4,
    };
    let frame = render(60, 60, &cam, &map);
    let mut w = stdout();
    frame.iter().for_each(
        |row| {
            w.write_all(&row.iter().map(|n| match n {
                0 => b' ',
                255 => b'#',
                _ => b'.'
            }).collect::<Vec<u8>>()).unwrap();
            w.write_all(&[b'\n']).unwrap();
        }
    )
}
