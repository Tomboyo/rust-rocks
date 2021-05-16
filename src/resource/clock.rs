use std::time::{Duration, Instant};

#[derive(Clone, Copy)]
pub struct Clock {
    pub now: Instant,
    pub dt: Duration,
}

impl Clock {
    pub fn new() -> Self {
        Clock {
            now: Instant::now(),
            dt: Duration::ZERO,
        }
    }

    pub fn update(&mut self, dt: Duration) {
        self.now += dt;
        self.dt = dt;
    }
}
