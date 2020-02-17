use nannou::ui::prelude::*;

pub struct WidgetIds {
    grid_height: widget::Id,
    grid_width: widget::Id,
    noise_scale: widget::Id,
    noise_seed: widget::Id,
    particle_lifetime: widget::Id,
    particle_max_weight: widget::Id,
    particle_min_weight: widget::Id,
    particle_auto_spawn_limit: widget::Id,
    particle_step_length: widget::Id,
    vector_magnitude: widget::Id,
    vector_spacing: widget::Id,
}

impl WidgetIds {
    pub fn new(ui: &mut Ui) -> Self {
        Self {
            grid_height: ui.generate_widget_id(),
            grid_width: ui.generate_widget_id(),
            noise_scale: ui.generate_widget_id(),
            noise_seed: ui.generate_widget_id(),
            particle_lifetime: ui.generate_widget_id(),
            particle_max_weight: ui.generate_widget_id(),
            particle_min_weight: ui.generate_widget_id(),
            particle_auto_spawn_limit: ui.generate_widget_id(),
            particle_step_length: ui.generate_widget_id(),
            vector_magnitude: ui.generate_widget_id(),
            vector_spacing: ui.generate_widget_id(),
        }
    }
}
