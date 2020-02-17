use super::enums::RedrawBackground;
use super::Model;
use crate::{flow_particle::FlowParticle, flow_vector::FlowVector, palette};
use nannou::geom::Rect;
use nannou::prelude::{App, Frame, Key, MouseButton, Update, Vector2, BLACK, WHITE};
use rand::prelude::ThreadRng;
use rand::Rng;

pub fn update(_app: &App, model: &mut Model, _update: Update) {
    update_ui(model);

    for fp in &mut model.flow_particles {
        if fp.age() > model.particle_lifetime {
            model.particle_cleanup_requested = true;
        }

        let nearest_angle =
            (model.nearest_angle_fn)(*fp.xy(), &model.window_rect, &model.flow_vectors);
        fp.update(nearest_angle);
    }

    if model.redraw_background != RedrawBackground::Complete {
        model.redraw_background = model.redraw_background.next();
    }

    if model.particle_cleanup_requested {
        let particle_lifetime = model.particle_lifetime.clone();

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
            model.regen_flow_vectors();
            model.reset();
        }
        _ => {}
    };
}

pub fn resized(_app: &App, model: &mut Model, _: Vector2) {
    model.redraw_background = RedrawBackground::Pending;
}

// pub fn new_random_particle(
//     window_rect: &Rect<f32>,
//     color_palette: &[&'static str],
//     line_cap: LineCap,
// ) -> FlowParticle {
//     let random_x = map_range(
//         rand::random(),
//         0.0,
//         1.0,
//         window_rect.left(),
//         window_rect.right(),
//     );
//     let random_y = map_range(
//         rand::random(),
//         0.0,
//         1.0,
//         window_rect.bottom(),
//         window_rect.top(),
//     );

//     FlowParticle::new(Vector2::new(random_x, random_y), color_palette, line_cap)
// }

// pub fn nearest_angle_in_grid(
//     xy: Vector2<f32>,
//     window_rect: &Rect<f32>,
//     flow_vectors: &[FlowVector],
// ) -> f32 {
//     let origin_x = window_rect.left() as f32 + VECTOR_SPACING;
//     let origin_y = window_rect.bottom() as f32 + VECTOR_SPACING;
//     let row_index = ((xy.x - origin_x) / VECTOR_SPACING).round() as usize;
//     let column_index = ((xy.y - origin_y) / VECTOR_SPACING).round() as usize;
//     let fv_index = row_index + column_index * GRID_W;

//     flow_vectors
//         .get(fv_index)
//         .map(|fv| fv.heading())
//         .unwrap_or(0.0)
// }

pub fn new_noise_opts(rng: &mut ThreadRng) -> (u32, f64) {
    let noise_seed = rng.gen_range(0, 100_000);
    let noise_scale = rng.gen_range(0.01, 0.1);

    (noise_seed, noise_scale)
}

pub fn update_ui(model: &mut Model) {
    let ui = &mut model.ui.set_widgets();

    // for value in slider(model.resolution as f32, 3.0, 15.0)
    //     .top_left_with_margin(20.0)
    //     .label("Resolution")
    //     .set(model.ids.resolution, ui)
    // {
    //     model.resolution = value as usize;
    // }
}

// pub fn slider(val: f32, min: f32, max: f32) -> widget::Slider<'static, f32> {
//     widget::Slider::new(val, min, max)
//         .w_h(200.0, 30.0)
//         .label_font_size(15)
//         .rgb(0.3, 0.3, 0.3)
//         .label_rgb(1.0, 1.0, 1.0)
//         .border(0.0)
// }
