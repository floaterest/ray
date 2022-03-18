use std::io::{Result, stdout, Write};
use std::f64::consts::*;
use std::ffi::OsStr;
use std::path::PathBuf;

use crossterm::{execute, terminal::SetTitle};
use clap::Parser;
use crossterm::event::{Event, KeyEvent, self, KeyCode};
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size};

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

    // let input = stdin();

    while let Ok(Event::Key(KeyEvent { code, .. })) = event::read() {
        match code {
            KeyCode::Up => cam.pitch(ROTATE),
            KeyCode::Down => cam.pitch(-ROTATE),
            KeyCode::Left => cam.yaw(-ROTATE),
            KeyCode::Right => cam.yaw(ROTATE),
            KeyCode::Char('q') => cam.roll(-ROTATE),
            KeyCode::Char('e') => cam.roll(ROTATE),

            KeyCode::Char('w') => cam.move_forward(MOVE, &map),
            KeyCode::Char('a') => cam.move_right(-MOVE, &map),
            KeyCode::Char('d') => cam.move_right(MOVE, &map),
            KeyCode::Char('s') => cam.move_forward(-MOVE, &map),

            KeyCode::Char('r') => {
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
            KeyCode::Esc => break,
            _ => continue,
        }
        let (width, height) = size()?;
        execute!(w, SetTitle(format_args!(
            "Size({},{},{},{}) | Pos({},{},{},{})",
            map.size.x,
            map.size.y,
            map.size.z,
            map.size.w,
            cam.pos.x as u32,
            cam.pos.y as u32,
            cam.pos.z as u32,
            cam.pos.w as u32,
        )))?;
        let mut scr = Screen::new(width as usize, height as usize, b' ');
        scr.render(&cam, &map);
        execute!(w, Clear(ClearType::All))?;
        for y in 0..scr.y {
            w.write_all(&scr[y])?;
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
        enable_raw_mode().unwrap();
        let mut w = stdout();
        match args.map.extension().unwrap().to_str() {
            Some("db") => render(&mut w, Map::from_file(args.map).unwrap()).unwrap(),
            Some("txt") => render(&mut w, Map::from_text(args.map).unwrap()).unwrap(),
            _ => eprintln!("Invalid map file, expected a txt or db file"),
        }
        disable_raw_mode().unwrap();
    }
}

fn main() {
    run();
}
