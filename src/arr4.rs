use std::fs::File;
use std::io::{Read, Result, Write};
use std::path::Path;
use crate::scanner::Scanner;

pub struct Arr4 {
    pub x: usize,
    pub y: usize,
    pub z: usize,
    pub w: usize,
    pub content: Vec<u8>,

    xy: usize,
    xyz: usize,
}

impl Arr4 {
    pub fn from(x: usize, y: usize, z: usize, w: usize, content: Vec<u8>) -> Self {
        Self {
            x,
            y,
            z,
            w,
            content,
            xy: x * y,
            xyz: x * y * z,
        }
    }

    pub fn from_text<P: AsRef<Path>>(path: P) -> Result<Self> {
        //! read data from text file
        let mut sc = Scanner::new(File::open(path)?);
        let (x, y, z, w) = (sc.usize(), sc.usize(), sc.usize(), sc.usize());
        let content: Vec<u8> = (0..x * y * z * w).map(|_| sc.u8()).collect();

        Ok(Self::from(x, y, z, w, content))
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut f = File::open(path)?;
        let mut buf = [0u8; 32];
        f.read(&mut buf)?;
        let (x, y, z, w) = (
            u64::from_be_bytes(buf[..8].try_into().unwrap()) as usize,
            u64::from_be_bytes(buf[8..16].try_into().unwrap()) as usize,
            u64::from_be_bytes(buf[16..24].try_into().unwrap()) as usize,
            u64::from_be_bytes(buf[24..].try_into().unwrap()) as usize,
        );
        let mut a = Self::with_capacity(x, y, z, w);
        f.read(&mut a.content);
        Ok(a)
    }

    pub fn with_capacity(x: usize, y: usize, z: usize, w: usize) -> Self {
        Self {
            x,
            y,
            z,
            w,
            content: Vec::with_capacity(x * y * z * w),
            xy: x * y,
            xyz: x * y * z,
        }
    }

    pub fn save_as<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        //! save into a binary file
        let mut f = File::create(path)?;
        // write x y z w
        for d in [self.x as u64, self.y as u64, self.z as u64, self.w as u64] {
            f.write(&d.to_be_bytes());
        }
        // write content
        f.write(&self.content)?;
        Ok(())
    }

    pub fn at(&self, x: usize, y: usize, z: usize, w: usize) -> &u8 {
        //! return &item at (x,y,z,w)
        &self.content[x + y * self.x + z * self.xy + w * self.xyz]
    }
}
