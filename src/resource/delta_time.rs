use std::{
    ops::Deref,
    time::{Duration, Instant},
};

pub struct DeltaTime {
    pub elapsed: Duration,
    from: Instant,
    to: Instant,
}

impl DeltaTime {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            from: now,
            to: now,
            elapsed: Duration::ZERO,
        }
    }

    pub fn since(&mut self) -> Self {
        let now = Instant::now();
        Self {
            from: self.to,
            to: now,
            elapsed: now.duration_since(self.to),
        }
    }

    pub fn as_f32(&self) -> f32 {
        (self.elapsed.as_micros() as f64 / 1_000_000.0) as f32
    }
}
