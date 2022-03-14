use crate::Vec4;

#[derive(Debug)]
pub struct Cam {
    /// current position
    pub pos: Vec4<f64>,
    /// azimuthal angle
    pub theta: f64,
    /// polar angle
    pub phi: f64,
    /// fov/2
    pub fov2: f64,
}
