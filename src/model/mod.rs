mod constants;
mod enums;
mod update;
mod view;

use crate::flow_vector::FlowVectorFieldBuilderFnOptions;
use crate::random_color::random_color;
pub use update::update;

use crate::flow_particle::{
    FlowParticle, FlowParticleBuilderFn, FlowParticleBuilderFnOptions, LineCap,
};
use crate::flow_vector::{new_right_hand_curve_flow_vectors, FlowVector, FlowVectorFieldBuilderFn};
use crate::palette;
use crate::widget_ids::WidgetIds;
use constants::{
    DEFAULT_GRID_H, DEFAULT_GRID_W, DEFAULT_MAX_WEIGHT, DEFAULT_MIN_WEIGHT,
    DEFAULT_PARTICLE_LIFETIME, DEFAULT_RESOLUTION_H, DEFAULT_RESOLUTION_W, DEFAULT_STEP_LENGTH,
    DEFAULT_VECTOR_MAGNITUDE, DEFAULT_VECTOR_SPACING,
};
use enums::{Background, RedrawBackground};
use nannou::geom::Rect;
use nannou::prelude::*;
use nannou::ui::prelude::*;
use rand::Rng;

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
    pub ui: Ui,
    pub vector_magnitude: f32,
    pub vector_spacing: f32,
    pub widget_ids: WidgetIds,
    pub window_rect: Rect<f32>,
    pub new_flow_particle_fn: FlowParticleBuilderFn,
    pub new_flow_vector_fn: FlowVectorFieldBuilderFn,
    pub nearest_angle_fn: Box<dyn Fn(Vector2<f32>, &Rect<f32>, &[FlowVector]) -> f32>,
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

        let flow_vectors = new_right_hand_curve_flow_vectors(
            &window_rect,
            DEFAULT_VECTOR_MAGNITUDE,
            DEFAULT_VECTOR_SPACING,
            DEFAULT_GRID_H,
            DEFAULT_GRID_W,
        );

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

        Self {
            _window,
            automatically_spawn_particles: false,
            background: Background::Vectors,
            color_palette: palette::MAGMA.to_vec(),
            flow_particles: Vec::with_capacity(64),
            flow_vectors,
            grid_height: DEFAULT_GRID_H,
            grid_width: DEFAULT_GRID_W,
            line_cap: LineCap::Square,
            mouse_xy: Vector2::new(0.0, 0.0),
            particle_cleanup_requested: false,
            redraw_background: RedrawBackground::Pending,
            rng,
            ui,
            widget_ids,
            window_rect,
            noise_scale,
            noise_seed,
            particle_lifetime: DEFAULT_PARTICLE_LIFETIME,
            particle_max_weight: DEFAULT_MAX_WEIGHT,
            particle_min_weight: DEFAULT_MIN_WEIGHT,
            particle_auto_spawn_limit: 400,
            particle_step_length: DEFAULT_STEP_LENGTH,
            vector_magnitude: DEFAULT_VECTOR_MAGNITUDE,
            vector_spacing: DEFAULT_VECTOR_SPACING,
            new_flow_particle_fn,
            new_flow_vector_fn: Box::new(|_| Vec::new()),
            nearest_angle_fn: Box::new(|_, _, _| 0.0),
        }
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

    pub fn regen_flow_vectors(&mut self) {
        self.flow_vectors = (self.new_flow_vector_fn)(FlowVectorFieldBuilderFnOptions {
            grid_h: self.grid_height,
            grid_w: self.grid_width,
            magnitude: self.vector_magnitude,
            noise_scale: self.noise_scale as f64,
            rng: &mut self.rng,
            vector_spacing: self.vector_spacing,
            window_rect: &self.window_rect,
        });
    }
}
