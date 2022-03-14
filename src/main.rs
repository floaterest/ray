use std::io::{Result, stdin, stdout, Write};
use std::env;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use crate::map::Map;
use crate::cam::Cam;
use crate::math::Vec4;
use crate::screen::Screen;

mod math;
mod cam;
mod map;
mod scanner;
mod screen;

fn run() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    let data = Map::from_file(String::from(&args[0]))?;
    let mut cam = Cam {
        pos: Vec4 { x: 1.5, y: 1.5, z: 1.5, w: 0.0 },
        theta: 0.0,
        phi: 0.0,
        fov2: 1.0,
    };
    let mut scr = Screen::new(200, 80, b' ');

    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode()?;
    let stdin = stdin();
    for c in stdin.keys() {
        match c? {
            Key::Up => cam.phi -= 0.1,
            Key::Down => cam.phi += 0.1,
            Key::Left => cam.theta += 0.1,
            Key::Right => cam.theta -= 0.1,
            Key::Char('q') => break,
            _ => {}
        }

        scr.render(&cam, &data);

        write!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1))?;
        for y in 0..scr.y {
            write!(stdout, "{}", termion::cursor::Goto(1, y as u16 + 1))?;
            stdout.write(&scr[y])?;
        }
    }
    stdout.write("\n".as_bytes())?;
    stdout.flush()?;
    Ok(())
}

fn main() {
    run().unwrap();
}
