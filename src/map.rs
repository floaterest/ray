use std::{fs::File, io::{Read, Result, Write}, path::Path};
use crate::scanner::Scanner;
use crate::Vec4;

pub struct Map {
    pub size: Vec4<usize>,
    pub spawn: Vec4<usize>,
    pub dest: Vec4<usize>,

    content: Vec<u8>,

    xy: usize,
    xyz: usize,
}

impl Map {
    pub fn from_text<P: AsRef<Path>>(path: P) -> Result<Self> {
        //! read data from text file
        let mut sc = Scanner::new(File::open(path)?);
        let mut read_vec4 = || Vec4 { x: sc.usize(), y: sc.usize(), z: sc.usize(), w: sc.usize() };
        let size = read_vec4();
        Ok(Self {
            size,
            spawn: read_vec4(),
            dest: read_vec4(),
            content: (0..size.x * size.y * size.z * size.w).map(|_| sc.u8()).collect(),

            xy: size.x * size.y,
            xyz: size.x * size.y * size.z,
        })
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut f = File::open(path)?;
        let mut buf = [0u8; 16];

        let mut read_vec4 = || -> Result<Vec4<usize>>{
            f.read(&mut buf)?;
            Ok(Vec4 {
                x: u32::from_be_bytes(buf[..4].try_into().unwrap()) as usize,
                y: u32::from_be_bytes(buf[4..8].try_into().unwrap()) as usize,
                z: u32::from_be_bytes(buf[8..12].try_into().unwrap()) as usize,
                w: u32::from_be_bytes(buf[12..].try_into().unwrap()) as usize,
            })
        };

        let size = read_vec4()?;
        let mut map = Self {
            size,
            spawn: read_vec4()?,
            dest: read_vec4()?,
            content: Vec::with_capacity(size.x * size.y * size.z * size.w),
            xy: size.x * size.y,
            xyz: size.x * size.y * size.z,

        };
        Read::by_ref(&mut f).take(map.content.capacity() as u64).read_to_end(&mut map.content)?;

        assert_ne!(map.content.len(), 0);
        Ok(map)
    }

    pub fn save_as<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        //! save into a binary file
        let mut f = File::create(path)?;
        // write size, spawn, dest
        for v in [self.size, self.spawn, self.dest] {
            for d in [v.x, v.y, v.z, v.w] {
                f.write(&(d as u32).to_be_bytes())?;
            }
        }
        // write content
        f.write(&self.content)?;
        Ok(())
    }

    pub fn index(&self, x: usize, y: usize, z: usize, w: usize) -> &u8 {
        //! return &item at (x,y,z,w)
        &self.content[x + y * self.size.x + z * self.xy + w * self.xyz]
    }

    pub fn is_inside(&self, x: f64, y: f64, z: f64, w: f64) -> bool {
        x >= 0.0 && y >= 0.0 && z >= 0.0 && w >= 0.0
            && x < self.size.x as f64 && y < self.size.y as f64 && z < self.size.z as f64 && w < self.size.w as f64
    }
}
