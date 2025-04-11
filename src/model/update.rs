use super::{enums::RedrawBackground, Model};
use macroquad::prelude::*;
use ::rand::Rng;

pub fn update(model: &mut Model) {
    handle_input(model);

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

    let valid_area = pad(model.window_rect, model.vector_spacing);

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

fn handle_input(model: &mut Model) {
    let (mx, my) = mouse_position();
    model.mouse_xy = Vec2::new(mx, my);

    if is_mouse_button_down(MouseButton::Left) {
        model.spawn_new_particle(model.mouse_xy);
    }
    if is_mouse_button_down(MouseButton::Right) {
        model.draw_particle_mode = true;
    }
    if is_mouse_button_released(MouseButton::Left) {
        model.draw_particle_mode = false;
    }

    if is_key_pressed(KeyCode::Space) {
        model.spawn_new_particle(model.get_random_xy());
    }
    if is_key_pressed(KeyCode::A) {
        model.automatically_spawn_particles = !model.automatically_spawn_particles;
    }
    if is_key_pressed(KeyCode::B) {
        model.background = model.background.next();
        model.redraw_background();
    }
    if is_key_pressed(KeyCode::C) {
        model.color_palette = model.color_palette.next();
    }
    if is_key_pressed(KeyCode::L) {
        model.line_cap = model.line_cap.next();
    }
    if is_key_pressed(KeyCode::N) {
        model.noise_seed = model.rng.gen_range(0..100_000);
        model.regen_flow_vectors();
    }
    if is_key_pressed(KeyCode::GraveAccent) {
        model.show_ui = !model.show_ui;
    }

    if window_was_resized(model) {
        model.redraw_background = RedrawBackground::Pending;
    }
}

fn window_was_resized(model: &mut Model) -> bool {
    let width = screen_width();
    let height = screen_height();
    let window_rect = Rect::new(0.0, 0.0, width, height);

    if window_rect != model.window_rect {
        model.window_rect = window_rect;
        return true;
    }
    false
}

fn pad(rect: Rect, pad: f32) -> Rect {
    Rect {
        x: rect.x - pad,
        y: rect.y - pad,
        w: rect.w + 2.0 * pad,
        h: rect.h + 2.0 * pad,
    }
}
