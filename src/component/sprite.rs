#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Sprite {
    pub width: u32,
    pub height: u32,
    pub kind: SpriteKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum SpriteKind {
    Asteroid,
    Bullet,
    Player,
    Title,
}

impl Sprite {
    pub fn default(kind: SpriteKind) -> Self {
        Self {
            width: 64,
            height: 64,
            kind,
        }
    }
}
