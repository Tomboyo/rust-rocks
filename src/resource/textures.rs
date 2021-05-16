use std::collections::HashMap;

use opengl_graphics::{Texture, TextureSettings};

use crate::component::SpriteKind;

pub type Textures = HashMap<SpriteKind, Texture>;

pub fn load_textures() -> Textures {
    let mut textures = HashMap::new();
    textures.insert(SpriteKind::Asteroid, load_texture("asteroid.bmp"));
    textures.insert(SpriteKind::Player, load_texture("player-ship.bmp"));
    textures.insert(SpriteKind::Bullet, load_texture("bullet.bmp"));
    textures.insert(SpriteKind::Title, load_texture("title.bmp"));
    textures
}

fn load_texture(name: &str) -> Texture {
    Texture::from_path("./resources/".to_owned() + name, &TextureSettings::new()).unwrap()
}
