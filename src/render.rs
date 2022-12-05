use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

use crate::{Camera, Map, Trace, Vec3};

const NUM_THREADS: usize = 8;

fn setup(w: usize, h: usize, cam: &Camera) -> (Vec3<f64>, Vec3<f64>, Vec3<f64>) {
    /*
    variable names are reference to
    https://upload.wikimedia.org/wikipedia/commons/b/b2/RaysViewportSchema.png
    in
    https://en.wikipedia.org/wiki/Ray_tracing_(graphics)#Calculate_rays_for_rectangular_viewport

    (assume d == 1)
    */
    // dist(top_ray, bottom_ray) == dist(highest_px, lowest_px) == height - 1
    let w = (w - 1) as f64; // hx
    let h = (h - 1) as f64; // hy

    // top x front == left
    let tn = cam.forward;
    let vn = cam.upward;
    let bn = tn * vn;// right

    // half of the frame's dimension
    let gx = cam.fov2.tan();
    let gy = gx * h / w;
    // top-left pixel
    let p = tn - bn * gx + vn * gy;
    // used to shift to the next pixel
    let qx = bn * (2.0 * gx / w);
    let qy = vn * (-2.0 * gy / h);

    (p, qx, qy)
}

pub fn render(width: usize, height: usize, cam: Arc<Camera>, map: Arc<Map>) -> Vec<Vec<u8>> {
    // get top-left ray and vectors for shifting
    let (p, dx, dy) = setup(width, height, &cam);

    let (n, r) = (height / NUM_THREADS, height % NUM_THREADS);
    let mid = n * (NUM_THREADS - r);
    // split height into NUM_THREADS parts (NUM_THREADS ranges), which is divided into
    // 1. `NUM_THREADS - r` ranges of length n
    // 2. `r` ranges of length n+1
    // therefore, 0..height is partitioned into NUM_THREADS sections
    let handles: Vec<JoinHandle<Vec<Vec<u8>>>> = (0..mid).step_by(n).map(|i| i..i + n).chain(
        (mid..height).step_by(n + 1).map(|i| i..i + n + 1)
    ).map(
        // need to clone cam and map for each thread
        |range| (range, Arc::clone(&cam), Arc::clone(&map))
    ).map(|(range, cam, map)| thread::spawn(
        move || range.map(|y| p + dy * y as f64).map(
            |p| (0..width).map(|x| p + dx * x as f64).map(
                |ray| Trace::new(&cam, &map, ray).last().unwrap()
            ).collect()
        ).collect()
    )).collect();

    let mut frame = Vec::with_capacity(height);
    for h in handles {
        frame.extend(h.join().unwrap())
    }
    frame
}
