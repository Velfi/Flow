pub mod constants;
pub mod enums;
pub mod update;

use crate::{
    flow_particle::{FlowParticle, FlowParticleBuilderFn, FlowParticleBuilderFnOptions},
    flow_vector::{FlowVector, FlowVectorFieldBuilder, FlowVectorFieldBuilderFn},
    lut_manager::LutManager,
};
use constants::{
    DEFAULT_AGING_RATE, DEFAULT_AUTO_SPAWN_PARTICLE_COUNT_LIMIT, DEFAULT_MAX_WEIGHT, DEFAULT_MIN_WEIGHT, DEFAULT_OUTSIDE_BOUNDS_TIMEOUT, 
    DEFAULT_PARTICLE_LIFETIME, DEFAULT_STEP_LENGTH, DEFAULT_VECTOR_MAGNITUDE,
    DEFAULT_VECTOR_SPACING,
};
use enums::{Background, RedrawBackground, ParticleShape};
use glam::Vec2;
use rand::Rng;
pub use update::update;

#[derive(Clone, Copy, Debug)]
pub struct SimpleRect {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl SimpleRect {
    pub fn from_w_h(w: f32, h: f32) -> Self {
        Self {
            left: -w / 2.0,
            right: w / 2.0,
            top: h / 2.0,
            bottom: -h / 2.0,
        }
    }

    pub fn contains(&self, point: Vec2) -> bool {
        point.x >= self.left && point.x <= self.right && point.y >= self.bottom && point.y <= self.top
    }
}

type NearestAngleFn = Box<dyn Fn(Vec2, &Model) -> f32>;

pub struct Model {
    pub automatically_spawn_particles: bool,
    pub background: Background,
    pub current_lut: String,
    pub draw_particle_mode: bool,
    pub flow_particles: Vec<FlowParticle>,
    pub flow_vector_field_builder_type: FlowVectorFieldBuilder,
    pub flow_vectors: Vec<FlowVector>,
    pub grid_height: usize,
    pub grid_width: usize,
    pub lut_manager: LutManager,
    pub mouse_xy: Vec2,
    pub nearest_angle_fn: NearestAngleFn,
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
    pub outside_bounds_timeout: f32,
    pub redraw_background: RedrawBackground,
    pub rng: rand::rngs::ThreadRng,
    pub show_ui: bool,
    pub vector_magnitude: f32,
    pub vector_spacing: f32,
    pub window_rect: SimpleRect,
    pub particle_shape: ParticleShape,
}

impl Model {
    pub fn new(window_size: Vec2) -> Self {
        let window_rect = SimpleRect::from_w_h(window_size.x, window_size.y);
        let mut rng = rand::thread_rng();
        let noise_seed = rng.gen_range(0..100_000);
        
        // Calculate grid size based on window dimensions
        let vector_spacing = DEFAULT_VECTOR_SPACING;
        let grid_width = (window_size.x / vector_spacing).ceil() as usize;
        let grid_height = (window_size.y / vector_spacing).ceil() as usize;
        
        // Adjust noise scale based on window size
        let noise_scale = 0.05_f64 * (1920.0_f64 / window_size.x as f64).max(1.0);
        
        let new_flow_particle_fn = Box::new(|options: FlowParticleBuilderFnOptions| {
            FlowParticle::new(
                options.age,
                options.aging_rate,
                options.color,
                options.step_length,
                options.weight,
                options.xy,
                options.shape,
            )
        });
        
        let lut_manager = LutManager::new();
        let available_luts = lut_manager.get_available_luts();
        let current_lut = available_luts.first().unwrap().clone();
        
        let mut model = Self {
            automatically_spawn_particles: true,
            background: Background::Vectors,
            current_lut,
            draw_particle_mode: false,
            flow_particles: Vec::with_capacity(DEFAULT_AUTO_SPAWN_PARTICLE_COUNT_LIMIT),
            flow_vectors: Vec::new(),
            grid_height,
            grid_width,
            lut_manager,
            mouse_xy: Vec2::new(0.0, 0.0),
            nearest_angle_fn: Box::new(nearest_angle_in_grid),
            new_flow_particle_fn,
            flow_vector_field_builder_type: FlowVectorFieldBuilder::Billow,
            new_flow_vector_fn: FlowVectorFieldBuilder::Billow.as_fn(),
            noise_scale,
            noise_seed,
            particle_auto_spawn_limit: DEFAULT_AUTO_SPAWN_PARTICLE_COUNT_LIMIT,
            particle_cleanup_requested: false,
            particle_lifetime: DEFAULT_PARTICLE_LIFETIME,
            particle_max_weight: DEFAULT_MAX_WEIGHT,
            particle_min_weight: DEFAULT_MIN_WEIGHT,
            particle_step_length: DEFAULT_STEP_LENGTH,
            outside_bounds_timeout: DEFAULT_OUTSIDE_BOUNDS_TIMEOUT,
            redraw_background: RedrawBackground::Pending,
            rng,
            show_ui: true,
            vector_magnitude: DEFAULT_VECTOR_MAGNITUDE,
            vector_spacing,
            window_rect,
            particle_shape: ParticleShape::Circle,
        };
        model.regen_flow_vectors();
        
        model
    }

    pub fn spawn_new_particle(&mut self, xy: Vec2) {
        let age = map_range(rand::random::<f32>(), 0.0, 1.0, 0.0, self.particle_lifetime);
        
        // Get color from LUT
        let lut_data = self.lut_manager.load_lut(&self.current_lut).unwrap();
        let color_index = (rand::random::<f32>() * 255.0) as usize;
        let color = egui::Color32::from_rgb(
            lut_data.red[color_index],
            lut_data.green[color_index],
            lut_data.blue[color_index],
        );
        
        let weight = map_range(
            rand::random::<f32>(),
            0.0,
            1.0,
            self.particle_min_weight,
            self.particle_max_weight,
        );
        let new_particle = (self.new_flow_particle_fn)(FlowParticleBuilderFnOptions {
            age,
            aging_rate: DEFAULT_AGING_RATE,
            color,
            step_length: self.rng.gen_range(0.0..1.0),
            weight,
            xy,
            shape: self.particle_shape,
        });
        self.flow_particles.push(new_particle);
    }

    pub fn get_random_xy(&self) -> Vec2 {
        let x = map_range(
            rand::random::<f32>(),
            0.0,
            1.0,
            self.window_rect.left,
            self.window_rect.right,
        );
        let y = map_range(
            rand::random::<f32>(),
            0.0,
            1.0,
            self.window_rect.bottom,
            self.window_rect.top,
        );
        Vec2::new(x, y)
    }

    pub fn get_origin(&self) -> (f32, f32) {
        let origin_x = self.window_rect.left + self.vector_spacing;
        let origin_y = self.window_rect.bottom + self.vector_spacing;
        (origin_x, origin_y)
    }

    pub fn regen_flow_vectors(&mut self) {
        self.flow_vectors = (self.new_flow_vector_fn)(self);
        self.background = Background::Vectors;
        self.redraw_background = RedrawBackground::Pending;
    }
}

fn map_range(value: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    (value - in_min) / (in_max - in_min) * (out_max - out_min) + out_min
}

pub fn nearest_angle_in_grid(xy: Vec2, model: &Model) -> f32 {
    let origin_x = model.window_rect.left + model.vector_spacing;
    let origin_y = model.window_rect.bottom + model.vector_spacing;
    let row_index = ((xy.x - origin_x) / model.vector_spacing).round() as i32;
    let column_index = ((xy.y - origin_y) / model.vector_spacing).round() as i32;
    
    // Clamp to valid grid bounds
    let row_index = row_index.max(0).min(model.grid_width as i32 - 1) as usize;
    let column_index = column_index.max(0).min(model.grid_height as i32 - 1) as usize;
    
    let fv_index = row_index + column_index * model.grid_width;
    
    // Additional safety check
    if fv_index < model.flow_vectors.len() {
        model.flow_vectors[fv_index].heading()
    } else {
        // This shouldn't happen with proper bounds checking, but fallback to 0.0
        0.0
    }
}
