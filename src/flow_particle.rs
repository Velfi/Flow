use crate::random_color::random_color;
use crate::PARTICLE_MAX_LIFETIME;
use nannou::prelude::*;

const MIN_AGE: f32 = 0.0;
const MAX_AGE: f32 = PARTICLE_MAX_LIFETIME / 2.0;
const STEP_LENGTH: f32 = 3.0;
const AGING: f32 = 0.1;
const MIN_WEIGHT: f32 = 3.0;
const MAX_WEIGHT: f32 = 10.0;

#[derive(Debug)]
pub struct FlowParticle {
    age: f32,
    color: Rgb<u8>,
    previous_xy: Vector2<f32>,
    weight: f32,
    xy: Vector2<f32>,
    line_cap: LineCap,
}

impl FlowParticle {
    pub fn new(xy: Vector2<f32>, color_palette: &[&'static str], line_cap: LineCap) -> Self {
        let random_age = map_range(rand::random(), 0.0, 1.0, MIN_AGE, MAX_AGE);
        let random_color = random_color(color_palette);
        let random_weight = map_range(rand::random(), 0.0, 1.0, MIN_WEIGHT, MAX_WEIGHT);

        FlowParticle {
            age: random_age,
            color: random_color,
            previous_xy: xy,
            weight: random_weight,
            xy,
            line_cap,
        }
    }

    pub fn update(&mut self, nearest_angle: f32) {
        self.age += AGING;
        self.previous_xy = self.xy;
        self.xy.x += STEP_LENGTH * f32::cos(nearest_angle.to_radians());
        self.xy.y += STEP_LENGTH * f32::sin(nearest_angle.to_radians());
    }

    pub fn draw(&self, draw: &app::Draw) {
        let d = draw
            .line()
            .color(self.color)
            .points(self.previous_xy, self.xy)
            .weight(self.weight);

        match self.line_cap {
            LineCap::Square => d.start_cap_square().end_cap_square(),
            LineCap::Round => d.start_cap_round().end_cap_round(),
        };
    }

    pub fn age(&self) -> f32 {
        self.age
    }

    pub fn xy(&self) -> &Vector2<f32> {
        &self.xy
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LineCap {
    Square,
    Round,
}

impl LineCap {
    pub fn next(self) -> Self {
        match self {
            Self::Square => Self::Round,
            Self::Round => Self::Square,
        }
    }
}
