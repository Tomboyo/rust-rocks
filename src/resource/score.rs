use std::ops::AddAssign;

#[derive(Debug)]
pub struct Score(u32);

impl Score {
    pub fn new() -> Self {
        Self(0)
    }
}

impl AddAssign<u32> for Score {
    fn add_assign(&mut self, rhs: u32) {
        self.0 += rhs;
    }
}
