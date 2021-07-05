mod flow_particle;
mod flow_vector;
mod model;
mod palette;
mod random_color;
mod widget_ids;

use log::info;
use model::{update, Model};

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

If some UI buttons and sliders seem to do nothing, it's because changes won't appear until you've pressed N to reset the game.
Also, mouse input can be a bit buggy on MacOS, sorry about that.
"#;

fn main() {
    let _ = dotenv::dotenv();
    env_logger::init();

    info!("Starting up the flow field...");
    info!("{}", CONTROLS);

    nannou::app(model).update(update).run();
}

fn model(app: &nannou::App) -> Model {
    Model::new(app)
}
