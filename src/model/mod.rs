mod constants;
mod enums;
mod update;
mod view;

use crate::{
    flow_particle::{FlowParticle, FlowParticleBuilderFn, FlowParticleBuilderFnOptions, LineCap},
    flow_vector::{new_simplex_noise_flow_vectors, FlowVector, FlowVectorFieldBuilderFn},
    palette,
    random_color::random_color,
    widget_ids::WidgetIds,
};
use constants::{
    DEFAULT_GRID_H, DEFAULT_GRID_W, DEFAULT_MAX_WEIGHT, DEFAULT_MIN_WEIGHT,
    DEFAULT_PARTICLE_LIFETIME, DEFAULT_RESOLUTION_H, DEFAULT_RESOLUTION_W, DEFAULT_STEP_LENGTH,
    DEFAULT_VECTOR_MAGNITUDE, DEFAULT_VECTOR_SPACING,
};
use enums::{Background, RedrawBackground};
use nannou::{
    geom::{Rect, Vector2},
    math::map_range,
    window, App, Ui,
};
use rand::Rng;
pub use update::update;

pub struct Model {
    pub _window: window::Id,
    pub automatically_spawn_particles: bool,
    pub background: Background,
    pub color_palette: Vec<&'static str>,
    pub flow_particles: Vec<FlowParticle>,
    pub flow_vectors: Vec<FlowVector>,
    pub grid_height: usize,
    pub grid_width: usize,
    pub line_cap: LineCap,
    pub mouse_xy: Vector2<f32>,
    pub nearest_angle_fn: Box<dyn Fn(Vector2<f32>, &Model) -> f32>,
    pub new_flow_particle_fn: FlowParticleBuilderFn,
    pub new_flow_vector_fn: FlowVectorFieldBuilderFn,
    pub noise_scale: f32,
    pub noise_seed: u32,
    pub particle_auto_spawn_limit: usize,
    pub particle_cleanup_requested: bool,
    pub particle_lifetime: f32,
    pub particle_max_weight: f32,
    pub particle_min_weight: f32,
    pub particle_step_length: f32,
    pub redraw_background: RedrawBackground,
    pub rng: rand::rngs::ThreadRng,
    pub show_ui: bool,
    pub ui: Ui,
    pub vector_magnitude: f32,
    pub vector_spacing: f32,
    pub widget_ids: WidgetIds,
    pub window_rect: Rect<f32>,
}

impl Model {
    pub fn new(app: &App) -> Self {
        let window_rect = Rect::from_w_h(DEFAULT_RESOLUTION_W as f32, DEFAULT_RESOLUTION_H as f32);

        let _window = app
            .new_window()
            .with_dimensions(DEFAULT_RESOLUTION_W, DEFAULT_RESOLUTION_H)
            .view(view::view)
            .mouse_moved(update::mouse_moved)
            .mouse_pressed(update::mouse_pressed)
            .key_pressed(update::key_pressed)
            .resized(update::resized)
            .build()
            .unwrap();

        let mut ui = app.new_ui().build().unwrap();

        let widget_ids = WidgetIds::new(&mut ui);

        let mut rng = rand::thread_rng();
        let noise_seed = rng.gen_range(0, 100_000);
        let noise_scale = rng.gen_range(0.01, 0.3);

        let new_flow_particle_fn = Box::new(|options: FlowParticleBuilderFnOptions| {
            FlowParticle::new(
                options.age,
                options.aging_rate,
                options.color,
                options.line_cap,
                options.step_length,
                options.weight,
                options.xy,
            )
        });
        // let flow_vectors = new_simplex_noise_flow_vectors(&window_rect, noise_seed, noise_scale);

        let mut model = Self {
            _window,
            automatically_spawn_particles: false,
            background: Background::Vectors,
            color_palette: palette::MAGMA.to_vec(),
            flow_particles: Vec::with_capacity(64),
            flow_vectors: Vec::new(),
            grid_height: DEFAULT_GRID_H,
            grid_width: DEFAULT_GRID_W,
            line_cap: LineCap::Square,
            mouse_xy: Vector2::new(0.0, 0.0),
            nearest_angle_fn: Box::new(nearest_angle_in_grid),
            new_flow_particle_fn,
            new_flow_vector_fn: Box::new(new_simplex_noise_flow_vectors),
            noise_scale,
            noise_seed,
            particle_auto_spawn_limit: 400,
            particle_cleanup_requested: false,
            particle_lifetime: DEFAULT_PARTICLE_LIFETIME,
            particle_max_weight: DEFAULT_MAX_WEIGHT,
            particle_min_weight: DEFAULT_MIN_WEIGHT,
            particle_step_length: DEFAULT_STEP_LENGTH,
            redraw_background: RedrawBackground::Pending,
            rng,
            show_ui: true,
            ui,
            vector_magnitude: DEFAULT_VECTOR_MAGNITUDE,
            vector_spacing: DEFAULT_VECTOR_SPACING,
            widget_ids,
            window_rect,
        };

        model.regen_flow_vectors();

        model
    }

    pub fn spawn_new_particle(&mut self, xy: Vector2<f32>) {
        let age = map_range(rand::random(), 0.0, 1.0, 0.0, self.particle_lifetime);
        let color = random_color(&self.color_palette);
        let weight = map_range(
            rand::random(),
            0.0,
            1.0,
            self.particle_min_weight,
            self.particle_max_weight,
        );
        let new_particle = (self.new_flow_particle_fn)(FlowParticleBuilderFnOptions {
            age,
            aging_rate: 0.1,
            color,
            line_cap: self.line_cap,
            step_length: 1.0,
            weight,
            xy,
        });

        self.flow_particles.push(new_particle);
    }

    pub fn reset(&mut self) {
        self.flow_particles = Vec::new();
        self.redraw_background = RedrawBackground::Pending;
    }

    pub fn get_random_xy(&self) -> Vector2<f32> {
        let x = map_range(
            rand::random(),
            0.0,
            1.0,
            self.window_rect.left(),
            self.window_rect.right(),
        );
        let y = map_range(
            rand::random(),
            0.0,
            1.0,
            self.window_rect.bottom(),
            self.window_rect.top(),
        );

        Vector2::new(x, y)
    }

    pub fn get_origin(&self) -> (f32, f32) {
        let origin_x = self.window_rect.left() as f32 + self.vector_spacing;
        let origin_y = self.window_rect.bottom() as f32 + self.vector_spacing;

        (origin_x, origin_y)
    }

    pub fn regen_flow_vectors(&mut self) {
        self.flow_vectors = (self.new_flow_vector_fn)(self);
    }
}

pub fn nearest_angle_in_grid(xy: Vector2<f32>, model: &Model) -> f32 {
    let origin_x = model.window_rect.left() as f32 + model.vector_spacing;
    let origin_y = model.window_rect.bottom() as f32 + model.vector_spacing;
    let row_index = ((xy.x - origin_x) / model.vector_spacing).round() as usize;
    let column_index = ((xy.y - origin_y) / model.vector_spacing).round() as usize;
    let fv_index = row_index + column_index * model.grid_width;

    model
        .flow_vectors
        .get(fv_index)
        .map(|fv| fv.heading())
        .unwrap_or(0.0)
}
