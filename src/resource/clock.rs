use std::time::Instant;

/// A clock which advances through time only while unpaused, and only when
/// instructed via the `advance` function. This is used to calculate delta-time
/// between frames.
pub struct Clock {
    /// True when the clock is paused, and false otherwise. While the clock is
    /// paused, time does not advance.
    pub is_paused: bool,
    since: Instant,
    /// The amount of time, in seconds, that has elapsed since the clock last
    /// advanced.
    pub delta: f32,
    /// The current time, which remains constant until the clock advances.
    pub now: Instant,
}

impl Clock {
    pub fn new() -> Self {
        Self {
            is_paused: false,
            since: Instant::now(),
            delta: 0.0,
            now: Instant::now(),
        }
    }

    pub fn pause(&mut self) {
        self.is_paused = true;
    }

    pub fn unpause(&mut self) {
        self.is_paused = false;
        self.since = Instant::now();
    }

    pub fn advance(&mut self) {
        if !self.is_paused {
            let now = Instant::now();
            let elapsed = now - self.since;
            self.delta = (elapsed.as_micros() as f64 / 1_000_000.0) as f32;
            self.now += elapsed;
            self.since = now;
        }
    }
}
