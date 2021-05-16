use piston::Size;

const WRAP_PAD: f32 = 64.0;
const DWRAP_PAD: f32 = 128.0;

#[derive(Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Copy, Clone)]
pub struct Bound {
    pub p0: Point,
    pub p1: Point,
    pub width: f32,
    pub height: f32,
}

#[derive(Copy, Clone)]
pub struct Bounds {
    pub inner: Bound,
    pub outer: Bound,
}

impl Bound {
    pub fn wrap_x(&self, x: f32) -> f32 {
        match x {
            x if x < self.p0.x => self.p1.x - (self.p0.x - x),
            x if x > self.p1.x => self.p0.x + x - self.p1.x,
            _ => x,
        }
    }

    pub fn wrap_y(&self, y: f32) -> f32 {
        match y {
            y if y < self.p0.y => self.p1.y - (self.p0.y - y),
            y if y > self.p1.y => self.p0.y + y - self.p1.y,
            _ => y,
        }
    }

    /// Returns true if the given coordinate pair is within these bounds.
    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.p0.x && x < self.p1.x && y >= self.p0.y && y < self.p1.y
    }
}

impl Bounds {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            inner: Bound {
                p0: Point { x, y },
                p1: Point {
                    x: x + width,
                    y: y + height,
                },
                width,
                height,
            },
            outer: Bound {
                p0: Point {
                    x: x - WRAP_PAD,
                    y: y - WRAP_PAD,
                },
                p1: Point {
                    x: x + width + DWRAP_PAD,
                    y: y + height + DWRAP_PAD,
                },
                width: width + DWRAP_PAD,
                height: height + DWRAP_PAD,
            },
        }
    }
}

impl From<Bound> for Size {
    fn from(bound: Bound) -> Self {
        Size {
            width: bound.width as f64,
            height: bound.height as f64,
        }
    }
}
