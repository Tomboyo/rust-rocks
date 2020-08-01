use std::path::Path;

use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::video::Window;
use sdl2::video::WindowContext;

use crate::position::Position;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Sprite {
    Asteroid,
    Bullet,
    Player,
}

pub struct Textures<'a> {
    asteroid: Texture<'a>,
    bullet: Texture<'a>,
    player: Texture<'a>,
    pub title: Texture<'a>,
}

impl <'a> Textures<'a> {
    pub fn new(
        texture_creator: &'a TextureCreator<WindowContext>
    ) -> Textures {
        Textures {
            asteroid: Self::load_texture(texture_creator, "asteroid.bmp"),
            bullet: Self::load_texture(texture_creator, "bullet.bmp"),
            player: Self::load_texture(texture_creator, "player-ship.bmp"),
            title: Self::load_texture(texture_creator, "title.bmp"),
        }
    }

    fn load_texture(
        texture_creator: &'a TextureCreator<WindowContext>,
        name: &str
    ) -> Texture<'a> {
        texture_creator
            .load_texture(Path::new("resources").join(name))
            .expect("Failed to load texture")
    }

    pub fn dimensions(
        &self,
        sprite: &Sprite
    ) -> (u32, u32) {
        let query = self.get_texture(sprite).query();
        (query.width, query.height)
    }

    pub fn get_texture(
        &self,
        sprite: &Sprite
    ) -> &Texture {
        match sprite {
            Sprite::Asteroid => &self.asteroid,
            Sprite::Bullet => &self.bullet,
            Sprite::Player => &self.player,
        }
    }
}

pub trait Renderable<'a> {
    fn position(&'a self) -> &'a Position;
    fn orientation(&self) -> f64;
    fn sprite(&'a self) -> &'a Sprite;
}

pub fn render<'a>(
    canvas: &mut Canvas<Window>,
    textures: &Textures,
    renderable: &'a dyn Renderable<'a>,
) -> Result<(), String> {
    let position = renderable.position();
    let orientation = renderable.orientation();
    let texture = textures.get_texture(renderable.sprite());
    let query = texture.query();
    let destination = Rect::new(
        (position.x - (query.width as f32 / 2.0)) as i32,
        (position.y - (query.height as f32 / 2.0)) as i32,
        query.width,
        query.height);
    canvas.copy_ex(
        &texture,
        None,
        destination,
        orientation as f64,
        None, // rotate around center of `destination`
        false,
        false)
}
