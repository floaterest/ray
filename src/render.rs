use std::cmp::Ordering;

use crate::{Camera, Map, Vec3};

const BORDER_SIZE: f64 = 0.03;

fn is_border(pos: &Vec3<f64>) -> bool {
    //! return whether pos it near a border (i.e. grid line)

    // if 2 of (x,y,z) is close to an integer => close to a grid line
    [pos.x, pos.y, pos.z].iter().filter(|&&n| (n - n.round()).abs() < BORDER_SIZE).count() >= 2
}

pub fn render(width: usize, height: usize, cam: &Camera, map: &Map) -> Vec<Vec<u8>> {
    /*
    variable names are reference to
    https://upload.wikimedia.org/wikipedia/commons/b/b2/RaysViewportSchema.png
    in
    https://en.wikipedia.org/wiki/Ray_tracing_(graphics)#Calculate_rays_for_rectangular_viewport

    (assume d == 1)
    */
    let mut frame = vec![vec![0; width]; height];
    // dist(top_ray, bottom_ray) == dist(highest_px, lowest_px) == height - 1
    let w = (width - 1) as f64; // hx
    let h = (height - 1) as f64; // hy

    // top x front == left
    let tn = cam.forward;
    let vn = cam.upward;
    let bn = vn * tn; // left, todo use right as in the image

    // half of the frame's dimension
    let gx = cam.fov2.tan();
    let gy = gx * h / w;
    // py: pov to (0,y) for y in [0, height)
    let mut py = tn + bn * gx + vn * gy;
    // used to shift to the next pixel
    let qx = bn * (-2.0 * gx / w); // todo use f64 * Vec3<f64> for scalar
    let qy = vn * (-2.0 * gy / h);

    // todo use Iter::map
    for y in 0..height {
        let mut ray = py.clone();
        for x in 0..width {
            // how often can a f64 be 0.0 ?
            // let delta = Vec3::compose(|i| if ray[i] == 0.0 { 1e30 } else { 1.0 / ray[i].abs() });
            // delta.x = distance from (x1, y1, z1) to (x1+1, y2, z2) along the ray
            let delta = Vec3::compose(|i| 1.0 / ray[i].abs()); // todo implement compose(&self, closure)
            // let step = Vec3::compose(|i| match ray[i] {
            //     0.0 => 0.0,
            //     f if f < 0.0 => -1.0,
            //     _ => 1.0
            // });
            // step.x = shift to the next block on x-axis
            let step = Vec3::compose(|i| if ray[i] < 0.0 { -1.0 } else { 1.0 });
            // the block that the ray is at (starts at camera)
            let mut block = Vec3::compose(|i| cam.pos[i].floor());
            // side.x = distance between pov.x and a side of block on x axis
            let mut side = Vec3::compose(|i| (cam.pos[i] - block[i]).abs() * delta[i]);
            // distance between pov and tip of the ray (i.e. ray's norm)
            let mut d = 0.0; // todo why 0? why not 1?
            while map.inside(block.x, block.y, block.z) {
                let hit = map.at(
                    block.x as usize, // todo implement Into
                    block.y as usize,
                    block.z as usize,
                );
                if *hit {
                    let target = Vec3::compose(|i| cam.pos[i] + ray[i] * d);
                    frame[y][x] = if is_border(&target) { 255 } else { 127 };
                    break;
                }
                // shift to the closest coord that has a integer value
                let min = (1..=3).min_by(|&i, &j| side[i].partial_cmp(&side[j]).unwrap_or(Ordering::Equal)).unwrap();
                d = side[min];
                side[min] += delta[min];
                block[min] += step[min];
            }
            ray += qx;
        }
        py += qy;
    }

    frame
}
