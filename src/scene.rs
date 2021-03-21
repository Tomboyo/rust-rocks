use crate::resource::input_events::InputEvents;

pub mod game;

pub trait Scene {
    fn run(&mut self, events: InputEvents);
}
