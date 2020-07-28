use std::time::Instant;

use crate::event::Event;
use crate::room::Room;
use crate::room::RoomTransition;
use crate::room::Context;

pub struct TitleRoom {}

impl TitleRoom {
    pub fn new(_context: &mut Context) -> Self {
        TitleRoom {}
    }
}

impl Room for TitleRoom{
    fn update(
        &mut self,
        context: &mut Context,
        events: Vec<Event>,
        _now: Instant
    ) -> Option<RoomTransition> {
        // Not quite "press any key;" press "Fire" to start game.
        if events.iter()
            .any(|e| matches!(e, Event::Fire)) {
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
