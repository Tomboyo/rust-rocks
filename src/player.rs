use crate::entity::Entity;
use crate::events::Event;

static MAX_SPEED: f32 = 5.0;

pub fn handle_event(player: &mut Entity, event: &Event) {
    match event {
        Event::LeftJoystick{ x, y } => {
            player.dx = x * MAX_SPEED;
            player.dy = y * MAX_SPEED;
        },
        Event::RightJoystick{ x: joy_x, y: joy_y } => {
            if (*joy_x, *joy_y) == (0, 0) {
                return;
            }

            // normalize x and y between -1 and 1
            let x =
                if *joy_x < 0 {
                    *joy_x as f64 / 32_768.0
                } else {
                    *joy_x as f64 / 32_767.0
                };
            let y =
                if *joy_y < 0 {
                    *joy_y as f64 / 32_768.0
                } else {
                    *joy_y as f64 / 32_767.0
                };

            // calculate the hypotenuse
            let h = ((x * x) + (y * y)).sqrt();

            // the y-axis maps -1.0 to -90, 0.0 => 0, and 1.0 => 90.
            // This is the correct angle when x >= 0.
            // When x <= 0, we can subtract that from 180.
            player.orientation =
                (y as f64 / h).asin() * (180.0 / std::f64::consts::PI)
                    * if x <= 0.0 { -1.0 } else { 1.0 }
                    + if x <= 0.0 { 180.0 } else { 0.0 };
        },
        _ => {}
    }
}
