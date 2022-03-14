use std::io::{Result, stdin, stdout, Write};
use std::env;
use termion::{event::Key, raw::IntoRawMode};
use termion::input::TermRead;

use crate::map::Map;
use crate::cam::Cam;
use crate::math::Vec4;
use crate::screen::Screen;

mod math;
mod cam;
mod map;
mod scanner;
mod screen;

fn run<W: Write>(w: &mut W) -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    let map = Map::from_file(String::from(&args[0]))?;
    let mut cam = Cam {
        pos: Vec4 { x: 4.5, y: 4.5, z: 4.5, w: 0.0 },
        theta: 0.0,
        phi: 0.0,
        fov2: 1.0,
    };
    let mut scr = Screen::new(500, 120, b' ');

    let input = stdin();
    for c in input.keys() {
        match c? {
            Key::Up => cam.phi -= 0.1,
            Key::Down => cam.phi += 0.1,
            Key::Left => cam.theta += 0.1,
            Key::Right => cam.theta -= 0.1,
            Key::Char('q') | Key::Char('\'') => break,
            _ => continue,
        }

        scr.render(&cam, &map);
        write!(w, "{}", termion::clear::All)?;
        for y in 0..scr.y {
            write!(w, "{}", termion::cursor::Goto(1, y as u16 + 1))?;
            w.write_all(&scr[y])?;
        }
    }
    Ok(())
}

fn main() {
    let output = stdout();
    run(&mut output.lock().into_raw_mode().unwrap()).unwrap();
}
