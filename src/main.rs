use std::path::PathBuf;
use std::io::Result;
use std::env;
use crate::cam::Cam;
use crate::math::{Ang3, Vec3};
use crate::reader::read_text;

mod math;
mod cam;
mod reader;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let (arr3, x, y, z) = read_text(&PathBuf::from(&args[1]))?;
    let cam = Cam {
        pos: Vec3 { x, y, z },
        ang: Ang3 { theta: 0.0, phi: 0.0 },
    };
    dbg!(&arr3,&cam);
    Ok(())
}
