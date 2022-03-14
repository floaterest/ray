use std::{fs::File, io::{Read, Result, Write}, path::Path};
use crate::scanner::Scanner;

pub struct Map {
    pub x: usize,
    pub y: usize,
    pub z: usize,
    pub w: usize,
    pub content: Vec<u8>,

    xy: usize,
    xyz: usize,
}

impl Map {
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

        let mut map = Self::with_capacity(x, y, z, w);
        Read::by_ref(&mut f).take((x * y * z * w) as u64).read_to_end(&mut map.content)?;

        assert_ne!(map.content.len(), 0);
        Ok(map)
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
            f.write(&d.to_be_bytes())?;
        }
        // write content
        f.write(&self.content)?;
        Ok(())
    }

    pub fn index(&self, x: usize, y: usize, z: usize, w: usize) -> &u8 {
        //! return &item at (x,y,z,w)
        &self.content[x + y * self.x + z * self.xy + w * self.xyz]
    }

    pub fn is_inside(&self, x: f64, y: f64, z: f64, w: f64) -> bool {
        x >= 0.0 && y >= 0.0 && z >= 0.0 && w >= 0.0
            && x < self.x as f64 && y < self.y as f64 && z < self.z as f64 && w < self.w as f64
    }
}
