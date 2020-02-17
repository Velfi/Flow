mod flow_particle;
mod flow_vector;
mod model;
mod palette;
mod random_color;
mod widget_ids;

use model::{update, Model};

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &nannou::App) -> Model {
    Model::new(app)
}
