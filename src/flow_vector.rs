use crate::{GRID_H, GRID_W, VECTOR_MAGNITUDE, VECTOR_SPACING};
use nannou::noise::{NoiseFn, Seedable};
use nannou::prelude::*;

const DEFAULT_V1: f32 = 0.0;
const DEFAULT_V2: f32 = VECTOR_MAGNITUDE;
// Values over 0.1 get pretty chaotic
const NOISE_SCALE: f64 = 0.05;

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

pub fn new_right_hand_curve_flow_vectors(window_rect: &Rect<f32>) -> Vec<FlowVector> {
    let origin_x = window_rect.left() as f32 + VECTOR_SPACING;
    let origin_y = window_rect.bottom() as f32 + VECTOR_SPACING;

    println!(
        "Creating vectors with origin x:{}, y:{}",
        origin_x, origin_y
    );

    (0..GRID_H)
        .map(move |column_index| {
            (0..GRID_W).map(move |row_index| {
                let xy = Point2::new(
                    (row_index as f32 * VECTOR_SPACING) + origin_x,
                    (column_index as f32 * VECTOR_SPACING) + origin_y,
                );

                let mut fv = FlowVector::new(xy);
                let a = (column_index as f32 / GRID_H as f32) * PI;
                fv.rotate(a.to_degrees());

                fv
            })
        })
        .flatten()
        .collect()
}

pub fn new_simplex_noise_flow_vectors(window_rect: &Rect<f32>, seed: u32) -> Vec<FlowVector> {
    let noise = nannou::noise::OpenSimplex::new().set_seed(seed);
    let origin_x = window_rect.left() as f32 + VECTOR_SPACING;
    let origin_y = window_rect.bottom() as f32 + VECTOR_SPACING;

    println!(
        "Creating vectors with origin x:{}, y:{}",
        origin_x, origin_y
    );

    (0..GRID_H)
        .map(move |column_index| {
            (0..GRID_W).map(move |row_index| {
                let xy = Point2::new(
                    (row_index as f32 * VECTOR_SPACING) + origin_x,
                    (column_index as f32 * VECTOR_SPACING) + origin_y,
                );

                let mut fv = FlowVector::new(xy);
                // let noise_value = noise.get([xy.x as f64, xy.y as f64]);
                let noise_value = noise.get([
                    row_index as f64 * NOISE_SCALE,
                    column_index as f64 * NOISE_SCALE,
                ]);
                let a = noise_value as f32 * TAU;
                fv.rotate(a.to_degrees());

                fv
            })
        })
        .flatten()
        .collect()
}
