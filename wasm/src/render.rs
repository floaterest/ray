use crate::{Camera, Map, Trace, Vec3};

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

fn rende(width: usize, height: usize, cam: Camera, map: Map) -> Vec<Vec<u8>> {
    // get top-left ray and vectors for shifting
    let (p, dx, dy) = setup(width, height, &cam);

    (0..height).map(|y| p + dy * y as f64).map(
        |p| (0..width).map(|x| p + dx * x as f64).map(
            |ray| Trace::new(&cam, &map, ray).last().unwrap()
        ).collect()
    ).collect()
}

pub fn render(width: usize, height: usize, cam: Camera, map: Map) -> Vec<u8>{
    rende(width,height,cam,map).into_iter().flat_map(
        |v|v.into_iter().flat_map(|px|[px,px,px,255])
    ).collect::<Vec<u8>>()
}