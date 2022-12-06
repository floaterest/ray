mod utils;
mod vec3;
mod camera;
mod render;
mod map;
mod trace;

use crate::{
    camera::Camera,
    map::Map,
    trace::Trace,
    vec3::Vec3,
};
use std::f64::consts::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm!");
}

#[wasm_bindgen]
pub fn render(width: usize, height: usize) -> Vec<u8>{
    let s = 9;
    let map = Map::new(Vec3::new(s, s, s), (0..s).map(
        |z| (0..s).map(
            |y| (0..s).map(|x| (x % (s - 1), y % (s - 1), z % (s - 1))).map(
                |(x, y, z)| if x * y + y * z + x * z == 0 { 255 } else { 0 }
                // |(x, y, z)| if x * y * z == 0 { 255 } else { 0 }
            ).collect()
        ).collect()
    ).collect());
    let cam = Camera {
        pos: Vec3::new(1.5, 1.5, 1.5),
        forward: Vec3::new(1.0, 1.0, 1.0).normal(),
        upward: Vec3::new(0.0, 0.0, 1.0).normal(),
        fov2: FRAC_PI_4,
    };
    render::render(width, height, cam, map)
}