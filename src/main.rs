mod flow_particle;
mod flow_vector;
mod palette;
mod random_color;

use flow_particle::{FlowParticle, LineCap};
use flow_vector::{new_right_hand_curve_flow_vectors, FlowVector};
use nannou::prelude::*;

pub const VECTOR_MAGNITUDE: f32 = 15.0;
pub const VECTOR_SPACING: f32 = 20.0 + VECTOR_MAGNITUDE;
pub const RESOLUTION_H: u32 = 1080;
pub const RESOLUTION_W: u32 = 1920;
pub const GRID_H: usize = (RESOLUTION_H as f32 / VECTOR_SPACING) as usize;
pub const GRID_W: usize = (RESOLUTION_W as f32 / VECTOR_SPACING) as usize;
pub const PARTICLE_MAX_LIFETIME: f32 = 200.0;
pub const AUTO_SPAWN_PARTICLE_COUNT_LIMIT: usize = 400;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    automatically_spawn_particles: bool,
    flow_particles: Vec<FlowParticle>,
    flow_vectors: Vec<FlowVector>,
    mouse_xy: Vector2<f32>,
    particle_cleanup_requested: bool,
    redraw_background: RedrawBackground,
    window_rect: Rect<f32>,
    color_palette: Vec<&'static str>,
    line_cap: LineCap,
}

fn model(app: &App) -> Model {
    let window_rect = Rect::from_w_h(RESOLUTION_W as f32, RESOLUTION_H as f32);

    let _window = app
        .new_window()
        .with_dimensions(RESOLUTION_W, RESOLUTION_H)
        .view(view)
        .mouse_moved(mouse_moved)
        .mouse_pressed(mouse_pressed)
        .key_pressed(key_pressed)
        .resized(resized)
        .build()
        .unwrap();

    let flow_vectors = new_right_hand_curve_flow_vectors(&window_rect);

    Model {
        _window,
        automatically_spawn_particles: false,
        flow_particles: Vec::with_capacity(64),
        flow_vectors,
        mouse_xy: Vector2::new(0.0, 0.0),
        particle_cleanup_requested: false,
        redraw_background: RedrawBackground::Pending,
        window_rect,
        color_palette: palette::MAGMA.to_vec(),
        line_cap: LineCap::Square,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for fp in &mut model.flow_particles {
        if fp.age() > PARTICLE_MAX_LIFETIME {
            model.particle_cleanup_requested = true;
        }

        let nearest_angle = nearest_angle(&fp.xy(), &model.window_rect, &model.flow_vectors);
        fp.update(nearest_angle);
    }

    if model.redraw_background != RedrawBackground::Complete {
        model.redraw_background = model.redraw_background.next();
    }

    if model.particle_cleanup_requested {
        model
            .flow_particles
            .retain(|fp| fp.age() < PARTICLE_MAX_LIFETIME);
        model.particle_cleanup_requested = false;
    }
    let left = model.window_rect.left();
    let right = model.window_rect.right();
    let bottom = model.window_rect.bottom();
    let top = model.window_rect.top();

    model.flow_particles.retain(|fp| {
        fp.xy().x > left || fp.xy().x < right || fp.xy().y > bottom || fp.xy().y < top
    });

    if model.automatically_spawn_particles
        && model.flow_particles.len() < AUTO_SPAWN_PARTICLE_COUNT_LIMIT
    {
        let new_particle =
            new_random_particle(&model.window_rect, &model.color_palette, &model.line_cap);
        model.flow_particles.push(new_particle);
    }
}

fn mouse_moved(_app: &App, model: &mut Model, pos: Vector2) {
    model.mouse_xy = pos;
}

fn mouse_pressed(_app: &App, model: &mut Model, button: MouseButton) {
    match button {
        MouseButton::Left => {
            let new_particle =
                FlowParticle::new(model.mouse_xy, &model.color_palette, &model.line_cap);
            model.flow_particles.push(new_particle);
        }
        MouseButton::Right => {
            model.flow_particles = Vec::new();
            model.redraw_background = RedrawBackground::Pending;
        }
        _ => {}
    }
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Space => {
            let new_particle =
                new_random_particle(&model.window_rect, &model.color_palette, &model.line_cap);
            model.flow_particles.push(new_particle);
        }
        Key::A => {
            model.automatically_spawn_particles = !model.automatically_spawn_particles;
        }
        Key::C => {
            model.color_palette = palette::new_random_palette();
            model.flow_particles = Vec::new();
            model.redraw_background = RedrawBackground::Pending;
        }
        Key::L => {
            model.line_cap = model.line_cap.next();
            println!("Switch line cap to: {:?}", model.line_cap);

            model.flow_particles = Vec::new();
            model.redraw_background = RedrawBackground::Pending;
        }
        _ => {}
    }
}

fn resized(_app: &App, model: &mut Model, _: Vector2) {
    model.redraw_background = RedrawBackground::Pending;
}

fn view(app: &App, model: &Model, frame: &Frame) {
    let draw = app.draw();

    if model.redraw_background == RedrawBackground::Pending {
        // order of loop is window_event -> update -> draw
        // that means this should never happen but if it does
        // then I messed up my logic.
        unreachable!();
    }

    if model.redraw_background == RedrawBackground::InProgress {
        draw.background().color(WHITE);

        for fv in &model.flow_vectors {
            fv.draw(&draw);
        }
    }

    for fp in &model.flow_particles {
        fp.draw(&draw);
    }

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}

// This enum only exists because I'm unsure of how to mutate the model during drawing or
// if that's even possible. This works around that by giving the draw phase a chance
// to respond to the redraw request before `update()` sets the state to Complete.
#[derive(PartialEq, Debug)]
enum RedrawBackground {
    Pending,
    InProgress,
    Complete,
}

impl RedrawBackground {
    pub fn next(&self) -> Self {
        match self {
            Self::Pending => Self::InProgress,
            Self::InProgress => Self::Complete,
            Self::Complete => Self::Complete,
        }
    }
}

fn new_random_particle(
    window_rect: &Rect<f32>,
    color_palette: &[&'static str],
    line_cap: &LineCap,
) -> FlowParticle {
    let random_x = map_range(
        rand::random(),
        0.0,
        1.0,
        window_rect.left(),
        window_rect.right(),
    );
    let random_y = map_range(
        rand::random(),
        0.0,
        1.0,
        window_rect.bottom(),
        window_rect.top(),
    );

    FlowParticle::new(Vector2::new(random_x, random_y), color_palette, line_cap)
}

fn nearest_angle(xy: &Vector2<f32>, window_rect: &Rect<f32>, flow_vectors: &[FlowVector]) -> f32 {
    let origin_x = window_rect.left() as f32 + VECTOR_SPACING;
    let origin_y = window_rect.bottom() as f32 + VECTOR_SPACING;
    let row_index = ((xy.x - origin_x) / VECTOR_SPACING).round() as usize;
    let column_index = ((xy.y - origin_y) / VECTOR_SPACING).round() as usize;
    let fv_index = row_index + column_index * GRID_W;

    flow_vectors
        .get(fv_index)
        .map(|fv| fv.heading())
        .unwrap_or(0.0)
}
