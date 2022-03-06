use std::path::PathBuf;
use std::io::Result;
use std::env;
use std::f64::consts::{FRAC_PI_2, FRAC_PI_4};
use crate::cam::Cam;
use crate::math::{Arr2, Vec3};
use crate::reader::read_text;

mod math;
mod cam;
mod reader;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    // let (arr3, x, y, z) = read_text(&PathBuf::from(&args[1]))?;
    let (_, x, y, z) = read_text(&PathBuf::from(&args[1]))?;
    let cam = Cam {
        pos: Vec3 { x, y, z },
        theta: FRAC_PI_4,
        phi: FRAC_PI_2,
        fov2: FRAC_PI_4,
    };
    // let mut scr: Arr2<char> = Arr2::new(100, 36, ' ');
    let scr: Arr2<char> = Arr2::new(100, 36, ' ');
    cam.picture(&scr);

    // dbg!(&arr3,&cam);
    Ok(())
}
