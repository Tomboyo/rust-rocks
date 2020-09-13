#[derive(Clone, Debug)]
pub struct Scores {
    previous: Option<u16>,
    current: Score
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Score {
    High   { score: u16 },
    Normal { score: u16, high: u16 }
}

impl Scores {
    pub fn init() -> Scores {
        Scores {
            previous: None,
            current: Score::High { score: 0 },
        }
    }

    pub fn new_score(&mut self, score: u16) -> &mut Self {
        self.previous = Some(match self.current {
            Score::High { score } => score,
            Score::Normal { score, .. } => score
        });

        self.current = self.previous
            .map(|high| if score > high {
                Score::High { score }
            } else {
                Score::Normal { score, high }
            })
            .unwrap_or(Score::High { score });
        
        self
    }

    pub fn score(&self) -> Score {
        self.current.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_first_score_is_a_high_score() {
        assert_eq!(
            Score::High { score: 0 },
            Scores::init().score())
    }

    #[test]
    fn test_new_high_score() {
        assert_eq!(
            Score::High { score: 1 },
            Scores::init().new_score(1).score())
    }

    #[test]
    fn test_new_normal_score() {
        assert_eq!(
            Score::Normal { score: 0, high: 1 },
            Scores::init()
                .new_score(1) // the high score
                .new_score(0) // the current score (< high score)
                .score())
    }
}
