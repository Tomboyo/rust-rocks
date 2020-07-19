use sdl2::EventPump;
use sdl2::event::Event as SdlEvent;

pub struct EventBus {
    gamepad: Gamepad
}

// SDL will emit changes for only one axis of a two-axis joystick if only one
// axis seems to have changed. We accumulate both axis into this struct so that
// we know the overall joystick state at any given time.
struct Gamepad {
    left_joystick: Joystick,
    right_joystick: RightJoystick,
}

struct Joystick {
    x: f32,
    y: f32,
}

struct RightJoystick {
    x: i16,
    y: i16,
}

pub enum Event {
    Quit,
    LeftJoystick { x: f32, y: f32 },
    RightJoystick { x: i16, y: i16 },
}

struct Flags {
    left_joy_changed: bool,
    right_joy_changed: bool,
}

impl EventBus {
    pub fn new() -> EventBus {
        EventBus {
            gamepad: Gamepad {
                left_joystick: Joystick { x: 0.0, y: 0.0 },
                right_joystick: RightJoystick { x: 0, y: 0 },
            },
        }
    }

    pub fn process_events(
        &mut self,
        event_pump: &mut EventPump
    ) -> Vec<Event> {
        let mut flags = Flags {
            left_joy_changed: false,
            right_joy_changed: false
        };

        let mut events = Vec::new();

        for event in event_pump.poll_iter() {
            match event {
                SdlEvent::Quit{ .. } =>
                    events.push(Event::Quit),
                SdlEvent::JoyAxisMotion { axis_idx: id, value, .. }
                if id <= 1 =>
                    self.handle_sdl2_joystick_event(&mut flags, id, value),
                SdlEvent::JoyAxisMotion { axis_idx: id, value, .. }
                if id > 1 =>
                    self.handle_sdl2_right_joystick(&mut flags, id, value),
                _ => {},
            };
        };

        if flags.left_joy_changed {
            events.push(
                Event::LeftJoystick {
                    x: self.gamepad.left_joystick.x,
                    y: self.gamepad.left_joystick.y,
                });
        };

        if flags.right_joy_changed {
            events.push(
                Event::RightJoystick {
                    x: self.gamepad.right_joystick.x,
                    y: self.gamepad.right_joystick.y,
                });
        };

        events
    }

    fn handle_sdl2_joystick_event(
        &mut self,
        flags: &mut Flags,
        id: u8,
        value: i16
    ) {
        let normal =
            if value >= 0 {
                value as f32 / 32_767.0
            } else {
                value as f32 / 32_768.0
            };
        
        match id {
            0 => self.gamepad.left_joystick.x = normal,
            1 => self.gamepad.left_joystick.y = normal,
            _ => panic!("id must be between 0 and 1 inclusive; got {}"),
        }
    
        flags.left_joy_changed = true;
    }

    fn handle_sdl2_right_joystick(
        &mut self,
        flags: &mut Flags,
        id: u8,
        value: i16
    ) { 
        match id {
            2 => self.gamepad.right_joystick.x = value,
            3 => self.gamepad.right_joystick.y = value,
            _ => panic!("id must be between 2 and 3 inclusive; got {}"),
        }

        flags.right_joy_changed = true;
    }
}
