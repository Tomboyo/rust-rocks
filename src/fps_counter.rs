use std::time::{Duration, Instant};

pub struct FpsCounter {
    since: Instant,
    frames: u32,
}

impl FpsCounter {
    pub fn new() -> Self {
        Self {
            since: Instant::now(),
            frames: 0,
        }
    }

    pub fn tick(&mut self) -> Option<u32> {
        self.frames += 1;
        let now = Instant::now();
        if self.since < now - Duration::from_secs(1) {
            let result = self.frames;
            self.frames = 0;
            self.since = now;
            Some(result)
        } else {
            None
        }
    }
}
