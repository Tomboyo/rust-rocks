use crate::resource::score::Score;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SceneEvent {
    Quit,
    GoToGame,
    PlayerHit { current_score: Score },
    GoToTitle,
}
