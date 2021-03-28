use std::error::Error;

use sdl2::controller::GameController;
use sdl2::GameControllerSubsystem;

pub struct Controllers {
    pub vec: Vec<GameController>,
}

impl Controllers {
    pub fn new(gcs: &GameControllerSubsystem) -> Result<Self, Box<dyn Error>> {
        let vec = (0..gcs.num_joysticks()?)
            .map(|x| gcs.open(x))
            .inspect(|x| match x {
                Ok(controller) => log::info!("Opened controller {}", controller.name()),
                Err(e) => log::warn!("Failed to open controller: {}", e),
            })
            .filter(|x| x.is_ok())
            .map(|x| x.unwrap())
            .collect();

        Ok(Self { vec })
    }
}
