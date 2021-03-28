#[derive(Copy, Clone)]
pub struct Bounds {
    pub width: f32,
    pub height: f32,
}

impl Bounds {
    /// Returns true if the given coordinate pair is within these bounds.
    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= 0.0 && x < self.width && y >= 0.0 && y < self.height
    }
}
