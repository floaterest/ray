use std::io::{Result, stdin, stdout, Write};
use std::env;
use std::f64::consts::*;
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

fn run<W: Write>(w: &mut W) -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    let map = Map::from_file(String::from(&args[0]))?;
    let mut cam = Cam {
        pos: Vec4 { x: 4.5, y: 4.5, z: 4.5, w: 0.0 },
        pov: to_vec3n(0.0, FRAC_PI_2),
        top: to_vec3n(0.0, 0.0),
        fov2: 1.0,
    };
    let (width, height) = terminal_size()?;
    let mut scr = Screen::new(width as usize, height as usize, b' ');

    let input = stdin();

    // let (cos,sin)=(0.1f64.cos(),0.1f64.sin());
    let (cos, sin) = (0.0, 1.0);

    for c in input.keys() {
        match c? {
            Key::Up => cam.rotate_top(cos, sin),
            Key::Down => cam.rotate_top(cos, -sin),
            Key::Left => cam.rotate_left(cos, sin),
            Key::Right => cam.rotate_left(cos, -sin),
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


    // let v = Vec3 { x: 1.0, y: 0.0, z: 0.0 };
    // let k = Vec3 { x: 0.0, y: 0.0, z: 1.0 };
    // let t = FRAC_PI_3;
    // let a = v * t.cos();
    // let b = (k * v) * t.sin();
    // let c = k * v.dot(&k) * (1.0 - t.cos());
    // let r = a + b + c;
    // dbg!((v, k, a, b, c, r));
}
