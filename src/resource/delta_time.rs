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

    /// Calculate the elapsed time since the last call to update
    pub fn update(&mut self) {
        let now = Instant::now();
        self.elapsed = now.duration_since(self.since);
        self.since = now;
    }

    pub fn as_f32(&self) -> f32 {
        (self.elapsed.as_micros() as f64 / 1_000_000.0) as f32
    }
}
