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
