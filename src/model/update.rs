use super::{enums::RedrawBackground, Model};
use crate::model::constants::DEFAULT_AUTO_SPAWN_PARTICLE_COUNT_LIMIT;
use log::info;
use nannou::{
    prelude::{App, Key, MouseButton, Update, Vec2},
    ui::prelude::*,
};
use rand::Rng;

pub fn update(_app: &App, model: &mut Model, _update: Update) {
    update_ui(model);

    for index in 0..model.flow_particles.len() {
        if model.flow_particles[index].age() > model.particle_lifetime {
            model.particle_cleanup_requested = true;
        }

        let nearest_angle = (model.nearest_angle_fn)(*model.flow_particles[index].xy(), model);
        model.flow_particles[index].update(nearest_angle, model.particle_step_length);
    }

    if model.redraw_background != RedrawBackground::Complete {
        model.redraw_background = model.redraw_background.next();
    }

    if model.particle_cleanup_requested {
        let particle_lifetime = model.particle_lifetime;

        model
            .flow_particles
            .retain(|fp| fp.age() < particle_lifetime);
        model.particle_cleanup_requested = false;
    }

    let valid_area = model.window_rect.pad(model.vector_spacing);

    model
        .flow_particles
        .retain(|fp| valid_area.contains(*fp.xy()));

    if model.automatically_spawn_particles
        && model.flow_particles.len() < model.particle_auto_spawn_limit
    {
        model.spawn_new_particle(model.get_random_xy());
    }

    if model.draw_particle_mode {
        model.spawn_new_particle(model.mouse_xy);
    }
}

pub fn mouse_moved(_app: &App, model: &mut Model, pos: Vec2) {
    model.mouse_xy = pos;
}

pub fn mouse_pressed(_app: &App, model: &mut Model, button: MouseButton) {
    match button {
        MouseButton::Left => {
            model.spawn_new_particle(model.mouse_xy);
        }
        MouseButton::Right => model.draw_particle_mode = true,
        _ => {}
    };
}

pub fn mouse_released(_app: &App, model: &mut Model, button: MouseButton) {
    if let MouseButton::Right = button {
        model.draw_particle_mode = false;
    }
}

pub fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Space => {
            model.spawn_new_particle(model.get_random_xy());
        }
        Key::A => {
            model.automatically_spawn_particles = !model.automatically_spawn_particles;
        }
        Key::B => {
            model.background = model.background.next();
            model.redraw_background();
        }
        Key::C => {
            model.color_palette = model.color_palette.next();
        }
        Key::L => {
            model.line_cap = model.line_cap.next();
        }
        Key::N => {
            model.noise_seed = model.rng.gen_range(0..100_000);
            model.regen_flow_vectors();
        }
        Key::Grave => {
            model.show_ui = !model.show_ui;
        }
        _ => {}
    };
}

pub fn resized(_app: &App, model: &mut Model, _: Vec2) {
    model.redraw_background = RedrawBackground::Pending;
}

pub fn update_ui(model: &mut Model) {
    if !model.show_ui {
        return;
    }

    //
    // This song and dance is complex and, as far as I can tell, necessary. The model owns the UI,
    // and the UI must be mutably borrowed in order to update all of the widgets.
    //
    // Rust's borrow checker is smart enough to know that mutably borrowing multiple fields of
    // a struct are fine, so long as those borrows are unique per field. Where it fails is when
    // calling a struct's method that requires `&self` or `&mut self`. The checker can't look into
    // that function to determine if the borrow is split. In order to sidestep this, I wrapped the
    // UI in an Option and check it out, like a library book, each time I need to update it.
    //
    // So long as it always gets put back at the end of the UI update phase, everything is hunky dory
    // AND I now get to call self-referencing methods on the model. Something similar could probably
    // be accomplished by a Mutex. This feels hacky and I'm open to suggestions if you think you
    // might have a better solution.
    //
    let mut ui = model.ui.take().unwrap();

    {
        let ui_cell = &mut ui.set_widgets();

        if button()
            .top_left_with_margin(20.0)
            .label("Hide Controls")
            .set(model.widget_ids.hide_ui, ui_cell)
            .was_clicked()
        {
            model.show_ui = false;
            info!("Controls hidden, press \"~\" to show them again")
        }

        if button()
            .down(10.0)
            .label("New Noise Seed")
            .set(model.widget_ids.noise_seed, ui_cell)
            .was_clicked()
        {
            model.noise_seed = model.rng.gen_range(0..100_000);
            model.regen_flow_vectors();
        }

        if button()
            .down(10.0)
            .label("Next Noise Function")
            .set(model.widget_ids.noise_fn, ui_cell)
            .was_clicked()
        {
            model.flow_vector_field_builder_type = model.flow_vector_field_builder_type.next();
            model.new_flow_vector_fn = model.flow_vector_field_builder_type.as_fn();
            model.regen_flow_vectors();
        }

        if let Some(value) = widget::Slider::new(model.noise_scale, 0.01, 0.1)
            .w_h(200.0, 30.0)
            .label_font_size(14)
            .rgb(0.3, 0.3, 0.3)
            .label_rgb(1.0, 1.0, 1.0)
            .border(1.0)
            .down(10.0)
            .label("Noise Scale")
            .set(model.widget_ids.noise_scale, ui_cell)
        {
            model.noise_scale = value;
            model.regen_flow_vectors();
        }

        if let Some(value) = slider(model.particle_lifetime, 10.0, 200.0)
            .down(10.0)
            .label("Particle Lifetime")
            .set(model.widget_ids.particle_lifetime, ui_cell)
        {
            model.particle_lifetime = value;
        }

        if let Some(value) = slider(model.particle_min_weight, 0.1, 1.0)
            .down(10.0)
            .label("Particle Min Thickness")
            .set(model.widget_ids.particle_min_weight, ui_cell)
        {
            model.particle_min_weight = value;
        }

        if let Some(value) = slider(model.particle_max_weight, 1.0, 100.0)
            .down(10.0)
            .label("Particle Max Thickness")
            .set(model.widget_ids.particle_max_weight, ui_cell)
        {
            model.particle_max_weight = value;
        }

        if let Some(value) = slider(model.particle_step_length, 0.001, 10.0)
            .down(10.0)
            .label("Particle Speed")
            .set(model.widget_ids.particle_step_length, ui_cell)
        {
            model.particle_step_length = value;
        }

        if button()
            .down(10.0)
            .label(&format!("Background: {}", model.background))
            .set(model.widget_ids.background, ui_cell)
            .was_clicked()
        {
            model.background = model.background.next();
            model.redraw_background = RedrawBackground::Pending;
        }

        if button()
            .down(10.0)
            .label(&format!("Line Cap: {}", model.line_cap))
            .set(model.widget_ids.line_cap, ui_cell)
            .was_clicked()
        {
            model.line_cap = model.line_cap.next();
        }

        if button()
            .down(10.0)
            .label(&format!("Palette: {}", model.color_palette))
            .set(model.widget_ids.palette, ui_cell)
            .was_clicked()
        {
            model.color_palette = model.color_palette.next();
        }

        if button()
            .down(10.0)
            .rgb(0.33, 0.08, 0.08)
            .label(&format!("Kill {} Particles", model.flow_particles.len()))
            .set(model.widget_ids.kill_all_particles, ui_cell)
            .was_clicked()
        {
            model.flow_particles = Vec::with_capacity(DEFAULT_AUTO_SPAWN_PARTICLE_COUNT_LIMIT);
        }
    }

    model.ui.replace(ui);
}

pub fn slider(val: f32, min: f32, max: f32) -> widget::Slider<'static, f32> {
    widget::Slider::new(val, min, max)
        .w_h(200.0, 30.0)
        .label_font_size(14)
        .rgb(0.3, 0.3, 0.3)
        .label_rgb(1.0, 1.0, 1.0)
        .border(1.0)
}

pub fn button() -> widget::Button<'static, widget::button::Flat> {
    widget::Button::new()
        .w_h(200.0, 30.0)
        .label_font_size(15)
        .rgb(0.3, 0.3, 0.3)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
}
