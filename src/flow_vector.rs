use nannou::prelude::*;

const DEFAULT_V1: f32 = 15.0;
const DEFAULT_V2: f32 = 0.0;

#[derive(Debug)]
pub struct FlowVector {
    xy: Vector2<f32>,
    vector: Vector2<f32>,
}

impl FlowVector {
    pub fn new(xy: Vector2<f32>) -> Self {
        Self {
            xy,
            vector: Vector2::new(DEFAULT_V1, DEFAULT_V2),
        }
    }

    pub fn rotate(&mut self, a: &f32) {
        let heading = self.heading().to_radians() + a.to_radians();
        let mag = self.mag();

        self.vector.x = heading.cos() * mag;
        self.vector.y = heading.sin() * mag;
    }

    fn mag(&self) -> f32 {
        f32::sqrt(self.mag_sq())
    }

    fn mag_sq(&self) -> f32 {
        let Vector2 { x, y } = self.vector;

        x.powi(2) + y.powi(2)
    }

    pub fn draw(&self, draw: &app::Draw) {
        let xy1 = self.xy;
        let xy2 = self.xy + self.vector;

        draw.line().points(xy1, xy2).color(BLACK);
        draw.ellipse().stroke(BLACK).radius(2.0).xy(xy2);
    }

    pub fn heading(&self) -> f32 {
        f32::atan2(self.vector.y, self.vector.x).to_degrees()
    }
}
