use nannou::noise::{NoiseFn, Seedable};
use nannou::prelude::*;
use rand::prelude::ThreadRng;

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

pub type FlowVectorFieldBuilderFn = Box<dyn Fn(FlowVectorFieldBuilderFnOptions) -> Vec<FlowVector>>;
pub struct FlowVectorFieldBuilderFnOptions<'a> {
    pub grid_h: usize,
    pub grid_w: usize,
    pub magnitude: f32,
    pub noise_scale: f64,
    pub rng: &'a mut ThreadRng,
    pub vector_spacing: f32,
    pub window_rect: &'a Rect<f32>,
}

pub fn new_right_hand_curve_flow_vectors(
    window_rect: &Rect<f32>,
    magnitude: f32,
    vector_spacing: f32,
    grid_h: usize,
    grid_w: usize,
) -> Vec<FlowVector> {
    let origin_x = window_rect.left() as f32 + vector_spacing;
    let origin_y = window_rect.bottom() as f32 + vector_spacing;

    println!(
        "Creating vectors with origin x:{}, y:{}",
        origin_x, origin_y
    );

    (0..grid_h)
        .map(move |column_index| {
            (0..grid_w).map(move |row_index| {
                let xy = Point2::new(
                    (row_index as f32 * vector_spacing) + origin_x,
                    (column_index as f32 * vector_spacing) + origin_y,
                );

                let mut fv = FlowVector::new(xy, magnitude);
                let a = (column_index as f32 / grid_h as f32) * PI;
                fv.rotate(a.to_degrees());

                fv
            })
        })
        .flatten()
        .collect()
}

pub fn new_simplex_noise_flow_vectors(
    window_rect: &Rect<f32>,
    seed: u32,
    scale: f64,
    magnitude: f32,
    vector_spacing: f32,
    grid_h: usize,
    grid_w: usize,
) -> Vec<FlowVector> {
    let noise = nannou::noise::OpenSimplex::new().set_seed(seed);
    let origin_x = window_rect.left() as f32 + vector_spacing;
    let origin_y = window_rect.bottom() as f32 + vector_spacing;

    println!(
        "Creating vectors with origin x:{}, y:{}",
        origin_x, origin_y
    );

    (0..grid_h)
        .map(move |column_index| {
            (0..grid_w).map(move |row_index| {
                let xy = Point2::new(
                    (row_index as f32 * vector_spacing) + origin_x,
                    (column_index as f32 * vector_spacing) + origin_y,
                );

                let mut fv = FlowVector::new(xy, magnitude);
                // let noise_value = noise.get([xy.x as f64, xy.y as f64]);
                let noise_value =
                    noise.get([row_index as f64 * scale, column_index as f64 * scale]);
                let a = noise_value as f32 * TAU;
                fv.rotate(a.to_degrees());

                fv
            })
        })
        .flatten()
        .collect()
}
