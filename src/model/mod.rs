mod constants;
mod enums;
mod update;
mod view;

use crate::{
    flow_particle::{FlowParticle, FlowParticleBuilderFn, FlowParticleBuilderFnOptions, LineCap},
    flow_vector::{FlowVector, FlowVectorFieldBuilder, FlowVectorFieldBuilderFn},
    palette::Palette,
    random_color::random_color,
    widget_ids::WidgetIds,
};
use constants::{
    DEFAULT_AGING_RATE, DEFAULT_AUTO_SPAWN_PARTICLE_COUNT_LIMIT, DEFAULT_GRID_H, DEFAULT_GRID_W,
    DEFAULT_MAX_WEIGHT, DEFAULT_MIN_WEIGHT, DEFAULT_NOISE_SCALE, DEFAULT_PARTICLE_LIFETIME,
    DEFAULT_RESOLUTION_H, DEFAULT_RESOLUTION_W, DEFAULT_STEP_LENGTH, DEFAULT_VECTOR_MAGNITUDE,
    DEFAULT_VECTOR_SPACING,
};
use enums::{Background, RedrawBackground};
use nannou::{
    geom::{Rect, Vec2},
    math::map_range,
    window, App, Ui,
};
use rand::Rng;
pub use update::update;

pub struct Model {
    pub _window: window::Id,
    pub automatically_spawn_particles: bool,
    pub background: Background,
    pub color_palette: Palette,
    pub draw_particle_mode: bool,
    pub flow_particles: Vec<FlowParticle>,
    pub flow_vector_field_builder_type: FlowVectorFieldBuilder,
    pub flow_vectors: Vec<FlowVector>,
    pub grid_height: usize,
    pub grid_width: usize,
    pub line_cap: LineCap,
    pub mouse_xy: Vec2,
    pub nearest_angle_fn: Box<dyn Fn(Vec2, &Model) -> f32>,
    pub new_flow_particle_fn: FlowParticleBuilderFn,
    pub new_flow_vector_fn: FlowVectorFieldBuilderFn,
    pub noise_scale: f64,
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
    pub ui: Option<Ui>,
    pub vector_magnitude: f32,
    pub vector_spacing: f32,
    pub widget_ids: WidgetIds,
    pub window_rect: Rect,
}

impl Model {
    pub fn new(app: &App) -> Self {
        let window_rect = Rect::from_w_h(DEFAULT_RESOLUTION_W as f32, DEFAULT_RESOLUTION_H as f32);

        let _window = app
            .new_window()
            .size(DEFAULT_RESOLUTION_W, DEFAULT_RESOLUTION_H)
            .view(view::view)
            .mouse_moved(update::mouse_moved)
            .mouse_pressed(update::mouse_pressed)
            .mouse_released(update::mouse_released)
            .key_pressed(update::key_pressed)
            .resized(update::resized)
            .build()
            .unwrap();

        let mut ui = Some(app.new_ui().build().unwrap());

        let widget_ids = WidgetIds::new(ui.as_mut().unwrap());

        let mut rng = rand::thread_rng();
        let noise_seed = rng.gen_range(0..100_000);

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
            color_palette: Default::default(),
            draw_particle_mode: false,
            flow_particles: Vec::with_capacity(DEFAULT_AUTO_SPAWN_PARTICLE_COUNT_LIMIT),
            flow_vectors: Vec::new(),
            grid_height: DEFAULT_GRID_H,
            grid_width: DEFAULT_GRID_W,
            line_cap: LineCap::Round,
            mouse_xy: Vec2::new(0.0, 0.0),
            nearest_angle_fn: Box::new(nearest_angle_in_grid),
            new_flow_particle_fn,
            flow_vector_field_builder_type: FlowVectorFieldBuilder::Billow,
            new_flow_vector_fn: FlowVectorFieldBuilder::Billow.as_fn(),
            noise_scale: DEFAULT_NOISE_SCALE,
            noise_seed,
            particle_auto_spawn_limit: DEFAULT_AUTO_SPAWN_PARTICLE_COUNT_LIMIT,
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

    pub fn spawn_new_particle(&mut self, xy: Vec2) {
        let age = map_range(rand::random(), 0.0, 1.0, 0.0, self.particle_lifetime);
        let color = random_color(&self.color_palette.as_colors());
        let weight = map_range(
            rand::random(),
            0.0,
            1.0,
            self.particle_min_weight,
            self.particle_max_weight,
        );
        let new_particle = (self.new_flow_particle_fn)(FlowParticleBuilderFnOptions {
            age,
            aging_rate: DEFAULT_AGING_RATE,
            color,
            line_cap: self.line_cap,
            step_length: self.rng.gen_range(0.0..1.0),
            weight,
            xy,
        });

        self.flow_particles.push(new_particle);
    }

    pub fn get_random_xy(&self) -> Vec2 {
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

        Vec2::new(x, y)
    }

    pub fn get_origin(&self) -> (f32, f32) {
        let origin_x = self.window_rect.left() as f32 + self.vector_spacing;
        let origin_y = self.window_rect.bottom() as f32 + self.vector_spacing;

        (origin_x, origin_y)
    }

    pub fn regen_flow_vectors(&mut self) {
        self.flow_vectors = (self.new_flow_vector_fn)(self);
        self.background = Background::Vectors;
        self.redraw_background = RedrawBackground::Pending;
    }

    pub fn redraw_background(&mut self) {
        self.redraw_background = RedrawBackground::Pending;
    }
}

pub fn nearest_angle_in_grid(xy: Vec2, model: &Model) -> f32 {
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
