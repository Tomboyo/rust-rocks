use super::WrapAround;

#[derive(Copy, Clone)]
pub struct Spatial {
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
    /// angle of orientation ("where it's pointing"), in radians
    pub angle_o: f32,
    pub wrap: WrapAround,
}
