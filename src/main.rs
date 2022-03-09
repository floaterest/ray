use std::path::PathBuf;
use std::io::{Result};
// use std::env;
// use std::f64::consts::{FRAC_PI_2, FRAC_PI_4};
use std::f64::consts::FRAC_PI_4;
use crate::cam::{Cam, render};
use crate::math::{Arr2, Vec3};
use crate::reader::read_text;

mod math;
mod cam;
mod reader;
mod block;

fn main() -> Result<()> {
    // let args: Vec<String> = env::args().skip(1).collect();

    // let (arr3, x, y, z) = read_text(&PathBuf::from(&args[0]))?;
    let (data, x, y, z) = read_text(&PathBuf::from("t.txt"))?;
    let cam = Cam {
        pos: Vec3 { x, y, z },
        // theta: FRAC_PI_4,
        // phi: FRAC_PI_2,
        theta: 0.0,
        phi: 0.0,
        fov2: FRAC_PI_4,
    };
    let mut scr: Arr2<u8> = Arr2::new(36, 36, b' ');
    render(&mut scr, &cam, &data);

    for y in 0..scr.y {
        for x in 0..scr.x {
            print!("{}", scr[y][x] as char);
        }
        print!("\n");
    }
    Ok(())
}
