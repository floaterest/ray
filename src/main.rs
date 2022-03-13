use std::path::{Path, PathBuf};
use std::io::{BufReader, Read, Result, stdin, stdout, Write};
use std::env;
use std::fs::File;
// use std::f64::consts::{FRAC_PI_2, FRAC_PI_4};
// use std::f64::consts::{FRAC_PI_4};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use crate::arr4::Arr4;
use crate::cam::{Cam, render};
use crate::math::{Arr2, Vec3, Vec4};
use crate::reader::read_text;

mod math;
mod cam;
mod reader;
mod arr4;
mod scanner;

fn run() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    let data = Arr4::from_file(String::from(&args[0]))?;
    let mut cam = Cam {
        pos: Vec4 { x: 1.5, y: 1.5, z: 1.5, w: 0.0 },
        // theta: FRAC_PI_4,
        // phi: FRAC_PI_2,
        theta: 0.0,
        phi: 0.0,
        fov2: 1.0,
    };
    let mut scr: Arr2<u8> = Arr2::new(200, 80, b' ');

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
        // stdout.flush().unwrap();
    }
    stdout.write("\n".as_bytes()).unwrap();
    stdout.flush().unwrap();
    Ok(())
}

fn main() {
    // let a = Arr4::from_text("test.txt").unwrap();
    // a.save_as("test.db").unwrap();
    // let a = Arr4::from_file("test.db").unwrap();
    // dbg!(a.x,a.y,a.z,a.w);
    run().unwrap();
}
