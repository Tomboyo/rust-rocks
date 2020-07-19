use std::collections::HashMap;

use sdl2::EventPump;
use sdl2::controller::Axis;
use sdl2::controller::GameController;
use sdl2::event::Event as SdlEvent;

pub struct EventBus {}

pub struct Joystick {
    x: i16,
    y: i16,
}

impl Joystick {
    // Get the joystick x reading normalized between -1.0 and 1.0, inclusive.
    pub fn unit_x_axis(&self) -> f32 {
        Joystick::normalize_axis(self.x)
    }

    // Get the joystick y reading normalized between -1.0 and 1.0, inclusive.
    pub fn unit_y_axis(&self) -> f32 {
        Joystick::normalize_axis(self.y)
    }

    // Convert an axis i16 reading to f64 between -1.0 and 1.0 inclusive
    fn normalize_axis(value: i16) -> f32 {
        if value >= 0 {
            value as f32 / 32_767.0
        } else {
            value as f32 / 32_768.0
        }
    }

    // Determines the angle of the joystick, if possible, based on the
    // coordinates of the joystick relative to the origin.
    //
    // The angle cannot be calculated when the coordinates are the origin
    // itself. We return None in this case only.
    //
    // This determines the angle of the joystick by finding the angle between
    // the x-axis and the hypotenuse of the special triangle formed by the
    // origin and the joystick coordinates.
    pub fn angle(&self) -> Option<f32> {
        if self.x == 0 && self.y == 0 {
            None
        } else {
            let (x, y) = (self.unit_x_axis(), self.unit_y_axis());

            let hypotenuse = ((x * x) + (y * y)).sqrt();
            let degrees = (y as f32 / hypotenuse).asin()
                * (180.0 / std::f32::consts::PI);
            // the y-axis maps -1.0 to -90, 0.0 => 0, and 1.0 => 90. This is the
            // correct angle when x >= 0. When x <= 0, we can subtract that from
            // 180.
            Some(
                if x <= 0.0 {
                    180.0 - degrees
                } else {
                    degrees
                })
        }
    }
}

pub enum Event {
    Quit,
    LeftJoystick { joystick: Joystick },
    RightJoystick { joystick: Joystick },
}

impl EventBus {
    pub fn new() -> EventBus {
        EventBus {}
    }

    pub fn process_events(
        &mut self,
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
                            joystick: EventBus::get_left_joystick_state(
                                controllers.get(&which).unwrap())
                        });
                },
                SdlEvent::JoyAxisMotion { which, axis_idx: id, .. }
                if (id == 2 || id == 3) && !right_joy_changed => {
                    right_joy_changed = true;
                    events.push(
                        Event::RightJoystick {
                            joystick: EventBus::get_right_joystick_state(
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
        Joystick {
            x: controller.axis(Axis::LeftX),
            y: controller.axis(Axis::LeftY),
        }
    }

    fn get_right_joystick_state(
        controller: &GameController,
    ) -> Joystick {
        Joystick {
            x: controller.axis(Axis::RightX),
            y: controller.axis(Axis::RightY),
        }
    }
}
