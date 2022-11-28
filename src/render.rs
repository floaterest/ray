use crate::{Camera, Map, Trace, Vec3};

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
    let bn = tn * vn;// right

    // half of the frame's dimension
    let gx = cam.fov2.tan();
    let gy = gx * h / w;
    // py: pov to (0,y) for y in [0, height)
    let mut py = tn + bn * gx + vn * gy;
    // used to shift to the next pixel
    let qx = bn * (2.0 * gx / w);
    let qy = vn * (-2.0 * gy / h);

    // (0..height).map(|y| (y, p + qy * y)).map(
    //     |(y, p)| (0..width).map(|x| (x, y, p + qx * x)).map(|(x, y, ray)| (
    //         x, y, ray,
    //         Vec3::compose(|i| 1.0 / ray[i].abs()), // delta
    //         Vec3::compose(|i| if ray[i] < 0.0 { -1.0 } else { 1.0 }), // step
    //         Vec3::compose(|i| cam.pos[i].floor()), // block
    //     )).map(|(x, y, ray, delta, step, block)| (
    //         x, y, ray, delta, step, block,
    //         Vec3::compose(|i| (cam.pos[i] - block[i]).abs() * delta[i]), // side
    //         0.0
    //     )).map(|(x, y, ray, delta, step, mut block, mut side, mut d)| {
    //         //
    //     })
    // ).collect();
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
            let block = Vec3::compose(|i| cam.pos[i].floor());
            // side.x = distance between pov.x and a side of block on x axis
            let side = Vec3::compose(|i| (cam.pos[i] - block[i]).abs() * delta[i]);
            // distance between pov and tip of the ray (i.e. ray's norm)
            let tr = Trace {
                map,
                cam,
                ray,
                delta,
                step,
                block,
                side,
                norm: 0.0,
            };
            frame[y][x] = tr.last().unwrap();
            ray += qx;
        }
        py += qy;
    }

    frame
}
