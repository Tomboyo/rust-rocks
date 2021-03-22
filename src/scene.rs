use crate::resource::input_events::InputEvents;

pub mod game;
pub mod scene_event;
pub mod score;
pub mod title;

pub trait Scene {
    fn run(&mut self, events: InputEvents);
}
