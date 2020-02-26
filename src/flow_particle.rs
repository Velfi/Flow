use nannou::prelude::*;

#[derive(Debug)]
pub struct FlowParticle {
    age: f32,
    aging_rate: f32,
    color: Rgba<u8>,
    line_cap: LineCap,
    previous_xy: Vector2<f32>,
    step_length: f32,
    weight: f32,
    xy: Vector2<f32>,
}

impl FlowParticle {
    pub fn new(
        age: f32,
        aging_rate: f32,
        color: Rgba<u8>,
        line_cap: LineCap,
        step_length: f32,
        weight: f32,
        xy: Vector2<f32>,
    ) -> Self {
        FlowParticle {
            age,
            aging_rate,
            color,
            line_cap,
            previous_xy: xy,
            step_length,
            weight,
            xy,
        }
    }

    pub fn update(&mut self, nearest_angle: f32, step_length: f32) {
        self.age += self.aging_rate;
        self.previous_xy = self.xy;
        self.xy.x += (self.step_length + step_length) * f32::cos(nearest_angle.to_radians());
        self.xy.y += (self.step_length + step_length) * f32::sin(nearest_angle.to_radians());
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

impl Default for FlowParticle {
    fn default() -> Self {
        let xy = Vector2::new(0.0, 0.0);

        FlowParticle {
            age: 0.0,
            aging_rate: 0.1,
            color: Rgba::new(0, 0, 0, 255),
            line_cap: LineCap::Round,
            previous_xy: xy,
            step_length: 1.0,
            weight: 1.0,
            xy,
        }
    }
}

pub type FlowParticleBuilderFn = Box<dyn Fn(FlowParticleBuilderFnOptions) -> FlowParticle>;

pub struct FlowParticleBuilderFnOptions {
    pub age: f32,
    pub aging_rate: f32,
    pub color: Rgba<u8>,
    pub line_cap: LineCap,
    pub step_length: f32,
    pub weight: f32,
    pub xy: Vector2<f32>,
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
