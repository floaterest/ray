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
const MOVE: f64 = 1.0;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// Map file (e.g. level.txt, level.db)
    #[clap(parse(from_os_str))]
    map: PathBuf,

    /// Convert map file from txt to db and exit
    #[clap(short, long)]
    convert: bool,
}

fn render<W: Write>(w: &mut W, map: Map) -> Result<()> {
    let mut cam = Cam {
        pos: Vec4 {
            x: map.spawn.x as f64 + 0.5,
            y: map.spawn.y as f64 + 0.5,
            z: map.spawn.z as f64 + 0.5,
            w: map.spawn.w as f64,
        },
        front: Vec3 { x: 1.0, y: 0.0, z: 0.0 },
        down: Vec3 { x: 0.0, y: 0.0, z: -1.0 },
        right: Vec3 { x: 0.0, y: -1.0, z: 0.0 },
        fov2: 1.0,
    };

    let input = stdin();

    for c in input.keys() {
        match c? {
            Key::Up => cam.pitch(ROTATE),
            Key::Down => cam.pitch(-ROTATE),
            Key::Left => cam.yaw(-ROTATE),
            Key::Right => cam.yaw(ROTATE),
            Key::Char('q') => cam.roll(-ROTATE),
            Key::Char('e') => cam.roll(ROTATE),

            Key::Char('w') => cam.move_forward(MOVE, &map),
            Key::Char('a') => cam.move_right(-MOVE, &map),
            Key::Char('d') => cam.move_right(MOVE, &map),
            Key::Char('s') => cam.move_forward(-MOVE, &map),

            Key::Char('r') => {
                cam.pos = Vec4 {
                    x: map.spawn.x as f64 + 0.5,
                    y: map.spawn.y as f64 + 0.5,
                    z: map.spawn.z as f64 + 0.5,
                    w: map.spawn.w as f64,
                };
                cam.front = Vec3 { x: 1.0, y: 0.0, z: 0.0 };
                cam.down = Vec3 { x: 0.0, y: 0.0, z: -1.0 };
                cam.right = Vec3 { x: 0.0, y: -1.0, z: 0.0 };
            }

            Key::Esc => break,
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

fn run() {
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
            Some("db") => render(&mut w, Map::from_file(args.map).unwrap()).unwrap(),
            Some("txt") => render(&mut w, Map::from_text(args.map).unwrap()).unwrap(),
            _ => eprintln!("Invalid map file, expected a txt or db file"),
        }
    }
}

fn main() {
    run();
}
