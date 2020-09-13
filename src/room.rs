pub mod game;
pub mod title;
pub mod score;

use std::time::Instant;

use sdl2::event::Event;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::input::ControllersMap;
use crate::render::Textures;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoomTransition {
    Game,
    Quit,
    Title,
    Score { score: u16 },
}

pub struct Context<'a> {
    pub canvas: &'a mut Canvas<Window>,
    pub textures: &'a Textures<'a>,
    pub controllers: &'a ControllersMap,
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
