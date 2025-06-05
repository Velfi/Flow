use super::{enums::RedrawBackground, Model};
use winit::event::{ElementState, MouseButton, KeyEvent};
use winit::keyboard::{Key, NamedKey};
use glam::Vec2;

pub fn update(model: &mut Model) {
    for index in 0..model.flow_particles.len() {
        if model.flow_particles[index].age() > model.particle_lifetime {
            model.particle_cleanup_requested = true;
        }

        let nearest_angle = (model.nearest_angle_fn)(*model.flow_particles[index].xy(), model);
        model.flow_particles[index].update(nearest_angle, model.particle_step_length);
        
        // Update time outside bounds
        let is_outside = !model.window_rect.contains(*model.flow_particles[index].xy());
        model.flow_particles[index].update_bounds_time(1.0, is_outside); // 1.0 frame delta
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

    // Kill particles that have been outside bounds for too long
    let outside_bounds_timeout = model.outside_bounds_timeout;
    model
        .flow_particles
        .retain(|fp| fp.time_outside_bounds() < outside_bounds_timeout);

    if model.automatically_spawn_particles
        && model.flow_particles.len() < model.particle_auto_spawn_limit
    {
        model.spawn_new_particle(model.get_random_xy());
    }

    if model.draw_particle_mode {
        model.spawn_new_particle(model.mouse_xy);
    }
}

pub fn mouse_moved(model: &mut Model, pos: Vec2) {
    model.mouse_xy = pos;
}

pub fn mouse_pressed(model: &mut Model, button: MouseButton, state: ElementState) {
    if state == ElementState::Pressed {
        match button {
            MouseButton::Left => {
                model.spawn_new_particle(model.mouse_xy);
            }
            MouseButton::Right => model.draw_particle_mode = true,
            _ => {}
        };
    }
}

pub fn mouse_released(model: &mut Model, button: MouseButton) {
    if let MouseButton::Right = button {
        model.draw_particle_mode = false;
    }
}

pub fn key_pressed(model: &mut Model, input: &KeyEvent) {
    if input.state == ElementState::Pressed {
        match &input.logical_key {
            Key::Named(NamedKey::Space) => {
                model.spawn_new_particle(model.get_random_xy());
            }
            Key::Named(NamedKey::Escape) => {
                std::process::exit(0);
            }
            Key::Character(c) if c == "/" || c == "?" => {
                model.show_ui = !model.show_ui;
            }
            _ => {}
        }
    }
}

pub fn resized(model: &mut Model) {
    model.redraw_background = RedrawBackground::Pending;
}
