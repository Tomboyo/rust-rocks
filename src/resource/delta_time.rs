use std::time::{Duration, Instant};

pub struct DeltaTime {
    pub elapsed: Duration,
    since: Instant,
}

impl DeltaTime {
    pub fn new() -> Self {
        Self {
            since: Instant::now(),
            elapsed: Duration::ZERO,
        }
    }

    pub fn since(&mut self) -> Self {
        let now = Instant::now();
        Self {
            since: now,
            elapsed: now.duration_since(self.since),
        }
    }

    pub fn as_f32(&self) -> f32 {
        (self.elapsed.as_micros() as f64 / 1_000_000.0) as f32
    }
}
