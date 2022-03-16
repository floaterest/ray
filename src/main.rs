use std::io::{Result, stdin, stdout, Write};
use std::env;
use std::f64::consts::*;
use std::fs::File;
use termion::{event::Key, raw::IntoRawMode, input::TermRead, terminal_size};

use crate::map::Map;
use crate::cam::Cam;
use crate::math::{to_vec3n, Vec3, Vec4};
use crate::screen::Screen;

mod math;
mod cam;
mod map;
mod scanner;
mod screen;

const ROTATE: f64 = FRAC_PI_3 / 10.0;

fn run<W: Write>(w: &mut W) -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    let map = Map::from_file(String::from(&args[0]))?;
    let mut cam = Cam {
        pos: Vec4 { x: 4.5, y: 4.5, z: 4.5, w: 0.0 },
        pov: Vec3 { x: 1.0, y: 0.0, z: 0.0 },
        top: Vec3 { x: 0.0, y: 0.0, z: 1.0 },
        fov2: 1.0,
    };

    let input = stdin();

    for c in input.keys() {
        match c? {
            Key::Up => cam.rotate_down(-ROTATE),
            Key::Down => cam.rotate_down(ROTATE),
            Key::Left => cam.rotate_left(ROTATE),
            Key::Right => cam.rotate_left(-ROTATE),

            Key::Char('r') => {
                cam.pov = Vec3 { x: 1.0, y: 0.0, z: 0.0 };
                cam.top = Vec3 { x: 0.0, y: 0.0, z: 1.0 };
            }

            Key::Char('q') | Key::Char('\'') => break,
            _ => continue,
        }

        let (width, height) = terminal_size()?;
        let mut scr = Screen::new(width as usize, height as usize, b' ');
        scr.render(&cam, &map);
        write!(w, "{}", termion::clear::All)?;
        for y in 0..scr.y {
            write!(w, "{}", termion::cursor::Goto(1, y as u16 + 1))?;
            w.write_all(&scr[y])?;
            // w.write_all("\n".as_bytes())?;
        }
    }
    Ok(())
}

fn main() {
    let output = stdout();
    run(&mut output.lock().into_raw_mode().unwrap()).unwrap();
}
