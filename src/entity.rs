pub struct Entity {
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
    pub orientation: f64,
}

impl Entity {
    pub fn new(x: i32, y: i32, dx: f32, dy: f32) -> Entity {
        Entity {
            x: x as f32,
            y: y as f32,
            dx,
            dy,
            orientation: 0.0,
        }
    }
}
