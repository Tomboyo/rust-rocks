use std::time::Instant;

use sdl2::event::Event;

use crate::room::Room;
use crate::room::RoomTransition;
use crate::room::Context;

pub struct TitleRoom {}

impl TitleRoom {
    pub fn new(_context: &mut Context) -> Self {
        TitleRoom {}
    }

    fn any_key(event: &Event) -> bool {
        match event {
            Event::ControllerButtonDown { .. } => true,
            Event::KeyDown { .. } => true,
            _ => false
        }
    }
}

impl Room for TitleRoom{
    fn update(
        &mut self,
        _context: &mut Context,
        events: Vec<Event>,
        _now: Instant
    ) -> Option<RoomTransition> {
        // Not quite "press any key;" press "Fire" to start game.
        if events.iter()
            .any(Self::any_key) {
                Some(RoomTransition::Game)
            } else {
                None
            }
    }

    fn render(
        &self,
        context: &mut Context
    ) {
        context.canvas.clear();

        context.canvas.copy(
            &context.textures.title,
            None,
            None).expect("Failed to render title!");
        
        context.canvas.present();
    }
}
