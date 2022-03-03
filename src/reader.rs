use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::PathBuf;
use std::str::FromStr;
use termion::input::TermRead;
use crate::linalg::Arr3;

fn take_from_line<T: FromStr>(mut r: &mut BufReader<File>, n: usize) -> Vec<T> {
    //! take n items from line
    TermRead::read_line(&mut r).unwrap().unwrap().split_ascii_whitespace().take(n)
        .map(|s| s.parse::<T>().ok().unwrap())
        .collect::<Vec<T>>()
}

pub fn read_text(fpath: &PathBuf) -> Result<(Arr3<bool>, f64, f64, f64)> {
    let mut r = BufReader::new(File::open(&fpath)?);

    // dimension
    let d = take_from_line::<usize>(&mut r, 3);
    // player initial position
    let pos = take_from_line::<f64>(&mut r, 3);
    // initialise map
    let mut blocks = Arr3::new(d[0], d[1], d[2], false);

    // read map data from file
    let mut y = 0;
    let mut z = 0;
    for line in r.lines() {
        let bytes = line.as_ref().unwrap().as_bytes();
        // if this line is not shorter than x size
        if bytes.len() >= d[0] {
            for x in 0..d[0] {
                // if is not air
                if bytes[x] != b'0' {
                    blocks[x][y][z] = true;
                }
            }
            y += 1;
            if y == d[1] {
                z += 1;
                y = 0;
            }
        }
    }

    Ok((blocks, pos[0], pos[1], pos[2]))
}
