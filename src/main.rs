use std::path::PathBuf;
use std::io::{Result, stdin, stdout, Write};
use std::env;
// use std::f64::consts::{FRAC_PI_2, FRAC_PI_4};
use std::f64::consts::{FRAC_PI_4};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use crate::cam::{Cam, render};
use crate::math::{Arr2, Vec3};
use crate::reader::read_text;

mod math;
mod cam;
mod reader;
mod block;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    let (data, x, y, z) = read_text(&PathBuf::from(&args[0]))?;
    // let (data, x, y, z) = read_text(&PathBuf::from("t.txt"))?;
    let mut cam = Cam {
        pos: Vec3 { x, y, z },
        // theta: FRAC_PI_4,
        // phi: FRAC_PI_2,
        theta: 0.0,
        phi: 0.0,
        fov2: 1.0,
    };
    let mut scr: Arr2<u8> = Arr2::new(100, 40, b' ');

    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let stdin = stdin();
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Up => cam.phi -= 0.1,
            Key::Down => cam.phi += 0.1,
            Key::Left => cam.theta += 0.1,
            Key::Right => cam.theta -= 0.1,
            Key::Char('q') => break,
            _ => {}
        }

        render(&mut scr, &cam, &data);
        write!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
        for y in 0..scr.y {
            write!(stdout, "{}", termion::cursor::Goto(1, y as u16 + 1)).unwrap();
            stdout.write(&scr[y]).unwrap();
        }
        stdout.flush().unwrap();
    }
    stdout.write("\n".as_bytes()).unwrap();
    stdout.flush().unwrap();
    Ok(())
}
