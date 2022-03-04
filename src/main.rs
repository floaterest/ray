use std::path::PathBuf;
use std::io::Result;
use std::env;
use crate::reader::read_text;

mod math;
mod cam;
mod reader;

fn main() -> Result<()> {
    let args:Vec<String> = env::args().collect();
    let (bs, x, y, z) = read_text(&PathBuf::from(&args[1]))?;
    dbg!(&bs,&x,&y,&z);
    Ok(())
}
