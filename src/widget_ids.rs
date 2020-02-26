use nannou::ui::prelude::*;

pub struct WidgetIds {
    pub grid_height: widget::Id,
    pub grid_width: widget::Id,
    pub noise_scale: widget::Id,
    pub noise_seed: widget::Id,
    pub particle_lifetime: widget::Id,
    pub particle_max_weight: widget::Id,
    pub particle_min_weight: widget::Id,
    pub particle_auto_spawn_limit: widget::Id,
    pub particle_step_length: widget::Id,
    pub vector_magnitude: widget::Id,
    pub vector_spacing: widget::Id,
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
