mod flow_particle;
mod flow_vector;
mod model;
mod palette;
mod random_color;

use log::info;
use macroquad::prelude::*;
use crate::model::Model;
use model::constants::{
    DEFAULT_RESOLUTION_H, DEFAULT_RESOLUTION_W,
};
use crate::model::view::view;
use crate::model::update::update;

const CONTROLS: &str = r#"
Left Click  - Spawn a new particle where you clicked
Right Click - "Draw" new particles where you click and drag

Space       - Spawn new particle in a random location
A           - Toggle On/Off. Automatically spawn a particle every frame unless too many already exist
B           - Clear the screen and change the background
C           - Switch to the next color palette
L           - Switch to the next line cap type
N           - Generate a new noise seed and reset the game
~           - Show or hide the UI
"#;

fn window_conf() -> Conf {
    Conf {
        window_title: "Flow".to_owned(),
        fullscreen: false,
        window_width: DEFAULT_RESOLUTION_W as i32,
        window_height: DEFAULT_RESOLUTION_H as i32,
        sample_count: 4,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let _ = dotenv::dotenv();
    env_logger::init();

    info!("Starting up the flow field...");
    info!("{}", CONTROLS);


    let mut model = Model::new();
    loop {
        update(&mut model);
        view(&model);
        next_frame().await
    }
}
