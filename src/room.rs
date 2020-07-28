pub mod game;
pub mod title;

use std::time::Instant;

use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::event::Event;
use crate::render::Textures;

pub enum RoomTransition {
    Game,
}

pub struct Context<'a> {
    pub canvas: &'a mut Canvas<Window>,
    pub textures: &'a Textures<'a>,
}

pub trait Room {
    fn update(
        &mut self,
        context: &mut Context,
        events: Vec<Event>,
        now: Instant
    ) -> Option<RoomTransition>;

    fn render(
        &self,
        context: &mut Context);
}
