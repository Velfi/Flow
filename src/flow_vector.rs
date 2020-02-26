use crate::model::Model;
use nannou::noise::{NoiseFn, Seedable};
use nannou::prelude::*;

#[derive(Debug)]
pub struct FlowVector {
    xy: Vector2<f32>,
    vector: Vector2<f32>,
}

impl FlowVector {
    pub fn new(xy: Vector2<f32>, magnitude: f32) -> Self {
        Self {
            xy,
            vector: Vector2::new(0.0, magnitude),
        }
    }

    pub fn rotate(&mut self, a: f32) {
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

pub type FlowVectorFieldBuilderFn = Box<dyn Fn(&Model) -> Vec<FlowVector>>;

pub fn new_right_hand_curve_flow_vectors(model: &Model) -> Vec<FlowVector> {
    let (origin_x, origin_y) = model.get_origin();

    (0..model.grid_height)
        .map(move |column_index| {
            (0..model.grid_width).map(move |row_index| {
                let xy = Point2::new(
                    (row_index as f32 * model.vector_spacing) + origin_x,
                    (column_index as f32 * model.vector_spacing) + origin_y,
                );

                let mut fv = FlowVector::new(xy, model.vector_magnitude);
                let a = (column_index as f32 / model.grid_height as f32) * PI;
                fv.rotate(a.to_degrees());

                fv
            })
        })
        .flatten()
        .collect()
}

pub fn new_simplex_noise_flow_vectors(model: &Model) -> Vec<FlowVector> {
    let noise = nannou::noise::OpenSimplex::new().set_seed(model.noise_seed);
    let (origin_x, origin_y) = model.get_origin();

    (0..model.grid_height)
        .map(move |column_index| {
            (0..model.grid_width).map(move |row_index| {
                let xy = Point2::new(
                    (row_index as f32 * model.vector_spacing) + origin_x,
                    (column_index as f32 * model.vector_spacing) + origin_y,
                );

                let mut fv = FlowVector::new(xy, model.vector_magnitude);
                let noise_value = noise.get([
                    row_index as f64 * model.noise_scale as f64,
                    column_index as f64 * model.noise_scale as f64,
                ]);
                let a = noise_value as f32 * TAU;
                fv.rotate(a.to_degrees());

                fv
            })
        })
        .flatten()
        .collect()
}
