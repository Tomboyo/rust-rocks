use std::{collections::HashMap, error::Error};

use sdl2::controller::GameController;
use sdl2::GameControllerSubsystem;

pub struct Controllers {
    pub map: HashMap<u32, GameController>,
}

impl Controllers {
    pub fn new(gcs: &GameControllerSubsystem) -> Result<Self, Box<dyn Error>> {
        let map = (0..gcs.num_joysticks()?)
            .map(|index| (index, gcs.open(index)))
            .inspect(|(_, result)| match result {
                Ok(controller) => log::info!("Opened controller {}", controller.name()),
                Err(e) => log::warn!("Failed to open controller: {}", e),
            })
            .filter(|(_, result)| result.is_ok())
            .map(|(index, result)| (index, result.unwrap()))
            .collect();

        Ok(Self { map })
    }
}

// impl Controllers for ControllersMap {
//     fn read_joystick(&self, controller_id: u32, axis_id: u8) -> Result<Joystick, String> {
//         use Axis::{LeftX, LeftY, RightX, RightY};
//         use WhichJoystick::{Left, Right};

//         match self.map.get(&controller_id) {
//             None => Err(format!("Unknown joystick {}", controller_id)),
//             Some(controller) => {
//                 if axis_id == LeftX as u8 || axis_id == LeftY as u8 {
//                     Ok(Joystick {
//                         x: controller.axis(LeftX),
//                         y: controller.axis(LeftY),
//                         which_joystick: Left,
//                     })
//                 } else if axis_id == RightX as u8 || axis_id == RightY as u8 {
//                     Ok(Joystick {
//                         x: controller.axis(RightX),
//                         y: controller.axis(RightY),
//                         which_joystick: Right,
//                     })
//                 } else {
//                     Err(format!("Unsupported joystick axis_id {}", axis_id))
//                 }
//             }
//         }
//     }
// }
