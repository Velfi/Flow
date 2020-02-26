use super::enums::RedrawBackground;
use super::Model;
use crate::palette;
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
        model.flow_particles[index].update(nearest_angle);
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
            model.color_palette = palette::new_random_palette();
            model.reset();
        }
        Key::L => {
            model.line_cap = model.line_cap.next();
            model.reset();
        }
        Key::N => {
            model.noise_seed = model.rng.gen_range(0, 100_000);
            model.noise_scale = model.rng.gen_range(0.01, 0.3);
            model.regen_flow_vectors();
            model.reset();
        }
        Key::Grave => {
            model.show_ui = !model.show_ui;
        }
        _ => println!(
            "Ignored pressed key {:?} because no handler has been set",
            key
        ),
    };
}

pub fn resized(_app: &App, model: &mut Model, _: Vector2) {
    model.redraw_background = RedrawBackground::Pending;
}

pub fn update_ui(model: &mut Model) {
    let ui = &mut model.ui.set_widgets();

    // grid_height: widget::Id,
    // grid_width: widget::Id,
    // noise_scale: widget::Id,
    // noise_seed: widget::Id,
    // particle_lifetime: widget::Id,
    // particle_max_weight: widget::Id,
    // particle_min_weight: widget::Id,
    // particle_auto_spawn_limit: widget::Id,
    // particle_step_length: widget::Id,
    // vector_magnitude: widget::Id,
    // vector_spacing: widget::Id,

    if let Some(value) = slider(model.noise_scale as f32, 0.01, 0.3)
        .top_left_with_margin(20.0)
        .label("Noise Scale")
        .set(model.widget_ids.noise_scale, ui)
    {
        model.noise_scale = value;
    }
}

pub fn slider(val: f32, min: f32, max: f32) -> widget::Slider<'static, f32> {
    widget::Slider::new(val, min, max)
        .w_h(200.0, 30.0)
        .label_font_size(15)
        .rgb(0.3, 0.3, 0.3)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
}
