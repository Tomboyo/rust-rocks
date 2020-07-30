pub struct Joystick {
    pub x: i16,
    pub y: i16,
    pub which_joystick: WhichJoystick
}

pub enum WhichJoystick {
    Left,
    Right,
}

impl Joystick {
    /// Get the joystick x reading normalized between -1.0 and 1.0, inclusive.
    pub fn normal_x(&self) -> f32 {
        Self::normalize_axis(self.x)
    }

    /// Get the joystick y reading normalized between -1.0 and 1.0, inclusive.
    pub fn normal_y(&self) -> f32 {
        Self::normalize_axis(self.y)
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
            let (x, y) = (self.normal_x(), self.normal_y());

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
