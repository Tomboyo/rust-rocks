macro_rules! button_press {
    ($button:ident) => {
        piston::Event::Input(
            piston::Input::Button(piston::ButtonArgs {
                state: piston::ButtonState::Press,
                button: piston::Button::Controller(piston::ControllerButton {
                    button: $button,
                    ..
                }),
                ..
            }),
            _,
        )
    };
}

macro_rules! button_release {
    ($button:ident) => {
        piston::Event::Input(
            piston::Input::Button(piston::ButtonArgs {
                state: piston::ButtonState::Release,
                button: piston::Button::Controller(piston::ControllerButton {
                    button: $button,
                    ..
                }),
                ..
            }),
            _,
        )
    };
}

macro_rules! axis {
    ($axis:ident, $position:ident) => {
        piston::Event::Input(
            piston::Input::Move(piston::Motion::ControllerAxis(piston::ControllerAxisArgs {
                axis: $axis,
                position: $position,
                ..
            })),
            _,
        )
    };
}

#[derive(Copy, Clone)]
pub struct ControllerState {
    pub left_thumb: (f64, f64),
    pub right_thumb: (f64, f64),
    pub right_bumper: bool,
}

impl ControllerState {
    pub fn new() -> Self {
        ControllerState {
            left_thumb: (0.0, 0.0),
            right_thumb: (0.0, 0.0),
            right_bumper: false,
        }
    }

    pub fn update_axis(&mut self, axis: u8, position: f64) {
        match axis {
            0 => self.left_thumb.0 = position,
            1 => self.left_thumb.1 = position,
            3 => self.right_thumb.0 = position,
            4 => self.right_thumb.1 = position,
            _ => (),
        }
    }

    pub fn press_right_bumper(&mut self) {
        self.right_bumper = true
    }

    pub fn release_right_bumper(&mut self) {
        self.right_bumper = false
    }
}
