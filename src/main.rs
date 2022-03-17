use std::io::{Result, stdin, stdout, Write};
use std::f64::consts::*;
use std::ffi::OsStr;
use std::path::PathBuf;

use termion::{event::Key, raw::IntoRawMode, input::TermRead, terminal_size};
use clap::Parser;

use crate::map::Map;
use crate::cam::Cam;
use crate::math::{Vec3, Vec4};
use crate::screen::Screen;

mod math;
mod cam;
mod map;
mod scanner;
mod screen;

const ROTATE: f64 = FRAC_PI_3 / 10.0;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// Map file (e.g. level.txt, level.db)
    #[clap(parse(from_os_str))]
    map: PathBuf,

    /// Convert map file to .db and exit
    #[clap(short, long)]
    convert: bool,
}

fn run<W: Write>(w: &mut W, map: Map) -> Result<()> {
    let mut cam = Cam {
        pos: Vec4 {
            x: map.spawn.x as f64 + 0.5,
            y: map.spawn.y as f64 + 0.5,
            z: map.spawn.z as f64 + 0.5,
            w: map.spawn.w as f64,
        },
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
    let args: Args = Args::parse();

    if args.convert {
        if args.map.extension() == Some(OsStr::new(&"txt")) {
            let map = Map::from_text(&args.map).unwrap();
            let mut path = PathBuf::from(&args.map);
            path.set_extension("db");
            map.save_as(path).unwrap();
        } else {
            eprintln!("Invalid input file, expected a txt file");
        }
    } else {
        let output = stdout();
        let mut w = output.lock().into_raw_mode().unwrap();
        match args.map.extension().unwrap().to_str() {
            Some("db") => run(&mut w, Map::from_file(args.map).unwrap()).unwrap(),
            Some("txt") => run(&mut w, Map::from_text(args.map).unwrap()).unwrap(),
            _ => eprintln!("Invalid map file, expected a txt or db file"),
        }
    }
}
