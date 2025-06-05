use glam::Vec2;
use egui::Color32;
use crate::model::enums::ParticleShape;

#[derive(Debug)]
pub struct FlowParticle {
    age: f32,
    aging_rate: f32,
    pub color: Color32,
    pub previous_xy: Vec2,
    step_length: f32,
    weight: f32,
    pub xy: Vec2,
    time_outside_bounds: f32,
    pub shape: ParticleShape,
}

impl FlowParticle {
    pub fn new(
        age: f32,
        aging_rate: f32,
        color: Color32,
        step_length: f32,
        weight: f32,
        xy: Vec2,
        shape: ParticleShape,
    ) -> Self {
        FlowParticle {
            age,
            aging_rate,
            color,
            previous_xy: xy,
            step_length,
            weight,
            xy,
            time_outside_bounds: 0.0,
            shape,
        }
    }

    pub fn update(&mut self, nearest_angle: f32, step_length: f32) {
        self.age += self.aging_rate;
        self.previous_xy = self.xy;
        self.xy.x += (self.step_length + step_length) * f32::cos(nearest_angle.to_radians());
        self.xy.y += (self.step_length + step_length) * f32::sin(nearest_angle.to_radians());
    }

    pub fn age(&self) -> f32 {
        self.age
    }

    pub fn xy(&self) -> &Vec2 {
        &self.xy
    }
    
    pub fn time_outside_bounds(&self) -> f32 {
        self.time_outside_bounds
    }
    
    pub fn update_bounds_time(&mut self, delta_time: f32, is_outside: bool) {
        if is_outside {
            self.time_outside_bounds += delta_time;
        } else {
            self.time_outside_bounds = 0.0;
        }
    }

    pub fn weight(&self) -> f32 {
        self.weight
    }
}

impl Default for FlowParticle {
    fn default() -> Self {
        let xy = Vec2::new(0.0, 0.0);
        FlowParticle {
            age: 0.0,
            aging_rate: 0.1,
            color: Color32::BLACK,
            previous_xy: xy,
            step_length: 1.0,
            weight: 1.0,
            xy,
            time_outside_bounds: 0.0,
            shape: ParticleShape::Circle,
        }
    }
}

pub type FlowParticleBuilderFn = Box<dyn Fn(FlowParticleBuilderFnOptions) -> FlowParticle>;

pub struct FlowParticleBuilderFnOptions {
    pub age: f32,
    pub aging_rate: f32,
    pub color: Color32,
    pub step_length: f32,
    pub weight: f32,
    pub xy: Vec2,
    pub shape: ParticleShape,
}
