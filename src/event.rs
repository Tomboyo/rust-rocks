mod joystick;

use std::collections::HashMap;

use sdl2::EventPump;
use sdl2::controller::Axis;
use sdl2::controller::GameController;
use sdl2::event::Event as SdlEvent;

use joystick::Joystick;

pub enum Event {
    Quit,
    LeftJoystick { joystick: Joystick },
    RightJoystick { joystick: Joystick },
}

pub fn process_events(
    event_pump: &mut EventPump,
    controllers: &HashMap<u32, GameController>,
) -> Vec<Event> {
    let mut left_joy_changed = false;
    let mut right_joy_changed = false;
    let mut events = Vec::new();

    for event in event_pump.poll_iter() {
        match event {
            SdlEvent::Quit{ .. } =>
                events.push(Event::Quit),
            SdlEvent::JoyAxisMotion { which, axis_idx: id, .. }
            if (id == 0 || id == 1) && !left_joy_changed => {
                left_joy_changed = true;
                events.push(
                    Event::LeftJoystick {
                        joystick: get_left_joystick_state(
                            controllers.get(&which).unwrap())
                    });
            },
            SdlEvent::JoyAxisMotion { which, axis_idx: id, .. }
            if (id == 2 || id == 3) && !right_joy_changed => {
                right_joy_changed = true;
                events.push(
                    Event::RightJoystick {
                        joystick: get_right_joystick_state(
                            controllers.get(&which).unwrap())
                    });
            },
            _ => {},
        };
    };

    events
}

fn get_left_joystick_state(
    controller: &GameController,
) -> Joystick {
    Joystick::new(
        controller.axis(Axis::LeftX),
        controller.axis(Axis::LeftY))
}

fn get_right_joystick_state(
    controller: &GameController,
) -> Joystick {
    Joystick::new(
        controller.axis(Axis::RightX),
        controller.axis(Axis::RightY))
}
