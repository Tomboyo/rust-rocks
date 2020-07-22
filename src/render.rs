use std::path::Path;

use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::video::Window;
use sdl2::video::WindowContext;

use crate::entity::Entity;

pub enum Sprite {
    Asteroid,
    Bullet,
    Player,
}

pub struct RenderSystem<'a> {
    asteroid: Texture<'a>,
    bullet: Texture<'a>,
    player: Texture<'a>
}

impl <'a> RenderSystem<'a> {
    pub fn new(
        texture_creator: &'a TextureCreator<WindowContext>
    ) -> RenderSystem {
        RenderSystem {
            asteroid: load_texture(texture_creator, "asteroid.bmp"),
            bullet: load_texture(texture_creator, "bullet.bmp"),
            player: load_texture(texture_creator, "player-ship.bmp"),
        }
    }

    pub fn render(
        &self,
        canvas: &mut Canvas<Window>,
        entity: &Entity,
    ) -> Result<(), String> {
        let texture = self.get_texture(&entity.sprite);
        let (width, height) = self.dimensions(&entity.sprite);
        let rectangle = Rect::new(
            (entity.x - (width as f32 / 2.0)) as i32,
            (entity.y - (height as f32 / 2.0)) as i32,
            width,
            height);
        canvas.copy_ex(
            &texture,
            None,
            rectangle,
            entity.orientation as f64,
            None, // rotate around center of `rectangle`
            false,
            false)
    }

    pub fn dimensions(
        &self,
        sprite: &Sprite
    ) -> (u32, u32) {
        let query = self.get_texture(sprite).query();
        (query.width, query.height)
    }

    fn get_texture(
        &self,
        sprite: &Sprite
    ) -> &Texture {
        match sprite {
            Sprite::Asteroid => &self.asteroid,
            Sprite::Bullet => &self.bullet,
            Sprite::Player => &self.player
        }
    }
}

fn load_texture<'a>(
    texture_creator: &'a TextureCreator<WindowContext>,
    name: &str
) -> Texture<'a> {
    texture_creator
        .load_texture(Path::new("resources").join(name))
        .expect("Failed to load texture")
}
