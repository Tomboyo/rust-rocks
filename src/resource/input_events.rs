use sdl2::event::Event;

pub struct InputEvents(Vec<Event>);

impl InputEvents {
    pub fn new(events: Vec<Event>) -> Self {
        Self(events)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Event> {
        self.0.iter()
    }
}
