use std::ops::AddAssign;

#[derive(Copy, Clone, Debug, Eq, PartialOrd, PartialEq)]
pub struct Score(u32);

impl Score {
    pub fn new() -> Self {
        Self(0)
    }
}

impl std::fmt::Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

impl AddAssign<u32> for Score {
    fn add_assign(&mut self, rhs: u32) {
        self.0 += rhs;
    }
}
