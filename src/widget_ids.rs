use nannou::ui::prelude::*;

pub struct WidgetIds {
    pub background: widget::Id,
    pub hide_ui: widget::Id,
    pub line_cap: widget::Id,
    pub noise_fn: widget::Id,
    pub noise_scale: widget::Id,
    pub noise_seed: widget::Id,
    pub palette: widget::Id,
    pub particle_lifetime: widget::Id,
    pub particle_max_weight: widget::Id,
    pub particle_min_weight: widget::Id,
    pub particle_step_length: widget::Id,
}

impl WidgetIds {
    pub fn new(ui: &mut Ui) -> Self {
        Self {
            background: ui.generate_widget_id(),
            hide_ui: ui.generate_widget_id(),
            line_cap: ui.generate_widget_id(),
            noise_fn: ui.generate_widget_id(),
            noise_scale: ui.generate_widget_id(),
            noise_seed: ui.generate_widget_id(),
            palette: ui.generate_widget_id(),
            particle_lifetime: ui.generate_widget_id(),
            particle_max_weight: ui.generate_widget_id(),
            particle_min_weight: ui.generate_widget_id(),
            particle_step_length: ui.generate_widget_id(),
        }
    }
}
