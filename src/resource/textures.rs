use std::{collections::HashMap, path::Path};

use sdl2::image::LoadTexture;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;

use crate::component::SpriteKind;

pub struct Textures {
    textures: HashMap<SpriteKind, Texture>,
}

impl<'a> Textures {
    pub fn new(texture_creator: &TextureCreator<WindowContext>) -> Self {
        let mut textures = HashMap::new();
        textures.insert(
            SpriteKind::Asteroid,
            Self::load_texture(texture_creator, "asteroid.bmp"),
        );
        textures.insert(
            SpriteKind::Player,
            Self::load_texture(texture_creator, "player-ship.bmp"),
        );
        textures.insert(
            SpriteKind::Bullet,
            Self::load_texture(texture_creator, "bullet.bmp"),
        );
        textures.insert(
            SpriteKind::Title,
            Self::load_texture(texture_creator, "title.bmp"),
        );
        Self { textures }
    }

    fn load_texture(texture_creator: &TextureCreator<WindowContext>, name: &str) -> Texture {
        texture_creator
            .load_texture(Path::new("resources").join(name))
            .expect("Failed to load texture")
    }

    pub fn get_texture(&self, sprite_kind: &SpriteKind) -> &Texture {
        self.textures.get(sprite_kind).unwrap()
    }
}
