pub mod game;

use std::time::Instant;

use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::event::Event;
use crate::render::Textures;

//TODO
pub enum RoomTransition {
    Title,
    Game,
}

pub struct Context<'a> {
    pub canvas: &'a mut Canvas<Window>,
    pub textures: &'a Textures<'a>,
}

pub trait Room {
    fn run(
        &mut self,
        events: Vec<Event>,
        now: Instant
    ) -> Option<RoomTransition>;
}
