use std::env;
use std::f64::consts::*;
use std::ffi::OsStr;
use std::io::{Result, stdout, Write};
use std::path::PathBuf;

use crate::cam::Cam;
use crate::map::Map;
use crate::math::{Vec3, Vec4};
use crate::screen::Screen;

mod math;
mod cam;
mod map;
mod scanner;
mod screen;

const ROTATE: f64 = FRAC_PI_3 / 10.0;
const MOVE: f64 = 1.0;

fn render<W: Write>(w: &mut W, map: Map) -> Result<()> {
    let mut cam = Cam {
        pos: Vec4 {
            x: map.spawn.x as f64 + 0.5,
            y: map.spawn.y as f64 + 0.5,
            z: map.spawn.z as f64 + 0.5,
            w: map.spawn.w as f64,
        },
        front: Vec3 { x: 1.0, y: 0.0, z: 0.0 },
        down: Vec3 { x: 0.0, y: 0.0, z: -1.0 },
        right: Vec3 { x: 0.0, y: -1.0, z: 0.0 },
        fov2: FRAC_PI_4,
    };
    let mut scr = Screen::new(20, 20, b' ');
    scr.render(&cam, &map);
    for y in 0..scr.y {
        w.write_all(&scr[y])?;
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    render(&mut stdout(), Map::from_text(file).unwrap()).unwrap();
}
