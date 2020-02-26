use super::enums::{Background, RedrawBackground};
use super::Model;
use nannou::prelude::{App, Key, MouseButton, Update, Vector2};
use nannou::ui::prelude::*;
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
    let left = model.window_rect.left();
    let right = model.window_rect.right();
    let bottom = model.window_rect.bottom();
    let top = model.window_rect.top();
    let vector_spacing = model.vector_spacing;

    model.flow_particles.retain(|fp| {
        fp.xy().x > left + vector_spacing
            && fp.xy().x < right - vector_spacing
            && fp.xy().y > bottom + vector_spacing
            && fp.xy().y < top - vector_spacing
    });

    if model.automatically_spawn_particles
        && model.flow_particles.len() < model.particle_auto_spawn_limit
    {
        model.spawn_new_particle(model.get_random_xy());
    }
}

pub fn mouse_moved(_app: &App, model: &mut Model, pos: Vector2) {
    model.mouse_xy = pos;
}

pub fn mouse_pressed(_app: &App, model: &mut Model, button: MouseButton) {
    match button {
        MouseButton::Left => {
            model.spawn_new_particle(model.mouse_xy);
        }
        MouseButton::Right => model.reset(),
        _ => {}
    };
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
            model.reset();
        }
        Key::C => {
            model.color_palette = model.color_palette.next();
        }
        Key::L => {
            model.line_cap = model.line_cap.next();
        }
        Key::N => {
            model.noise_seed = model.rng.gen_range(0, 100_000);
            model.regen_flow_vectors();
            model.reset();
        }
        Key::Grave => {
            model.show_ui = !model.show_ui;
        }
        _ => {}
    };
}

pub fn resized(_app: &App, model: &mut Model, _: Vector2) {
    model.redraw_background = RedrawBackground::Pending;
}

pub fn update_ui(model: &mut Model) {
    let ui = &mut model.ui.set_widgets();

    if !model.show_ui {
        return;
    }

    if button()
        .top_left_with_margin(20.0)
        .label("Hide Controls")
        .set(model.widget_ids.hide_ui, ui)
        .was_clicked()
    {
        model.show_ui = false;
        println!("Controls hidden, press \"~\" to show them again")
    }

    if button()
        .down(10.0)
        .label("New Noise Seed")
        .set(model.widget_ids.noise_seed, ui)
        .was_clicked()
    {
        model.noise_seed = model.rng.gen_range(0, 100_000);
        // TODO how the heck do I rewrite this to make this callable?
        // model.regen_flow_vectors();
        model.background = Background::Vectors;
        model.redraw_background = RedrawBackground::Pending;
    }

    if button()
        .down(10.0)
        .label("Next Noise Function")
        .set(model.widget_ids.noise_fn, ui)
        .was_clicked()
    {
        model.flow_vector_field_builder_type = model.flow_vector_field_builder_type.next();
        model.new_flow_vector_fn = model.flow_vector_field_builder_type.as_fn();
        model.background = Background::Vectors;
        model.redraw_background = RedrawBackground::Pending;
    }

    if let Some(value) = widget::Slider::new(model.noise_scale, 0.01, 0.1)
        .w_h(200.0, 30.0)
        .label_font_size(14)
        .rgb(0.3, 0.3, 0.3)
        .label_rgb(1.0, 1.0, 1.0)
        .border(1.0)
        .down(10.0)
        .label("Noise Scale")
        .set(model.widget_ids.noise_scale, ui)
    {
        model.noise_scale = value;
    }

    if let Some(value) = slider(model.particle_lifetime, 10.0, 200.0)
        .down(10.0)
        .label("Particle Lifetime")
        .set(model.widget_ids.particle_lifetime, ui)
    {
        model.particle_lifetime = value;
    }

    if let Some(value) = slider(model.particle_min_weight, 0.1, 1.0)
        .down(10.0)
        .label("Particle Min Thickness")
        .set(model.widget_ids.particle_min_weight, ui)
    {
        model.particle_min_weight = value;
    }

    if let Some(value) = slider(model.particle_max_weight, 1.0, 100.0)
        .down(10.0)
        .label("Particle Max Thickness")
        .set(model.widget_ids.particle_max_weight, ui)
    {
        model.particle_max_weight = value;
    }

    if let Some(value) = slider(model.particle_step_length, 0.001, 10.0)
        .down(10.0)
        .label("Particle Speed")
        .set(model.widget_ids.particle_step_length, ui)
    {
        model.particle_step_length = value;
    }

    if button()
        .down(10.0)
        .label("Background")
        .set(model.widget_ids.background, ui)
        .was_clicked()
    {
        model.background = model.background.next();
        model.redraw_background = RedrawBackground::Pending;
    }

    if button()
        .down(10.0)
        .label("Line Caps")
        .set(model.widget_ids.line_cap, ui)
        .was_clicked()
    {
        model.line_cap = model.line_cap.next();
    }

    if button()
        .down(10.0)
        .label("Random Palette")
        .set(model.widget_ids.palette, ui)
        .was_clicked()
    {
        model.color_palette = model.color_palette.next();
    }
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
