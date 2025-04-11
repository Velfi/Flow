use std::f32::consts::{PI, TAU};
use crate::model::Model;
use log::debug;
use macroquad::prelude::*;
use noise::{BasicMulti, Billow, Fbm, HybridMulti, NoiseFn, OpenSimplex};

#[derive(Debug)]
pub struct FlowVector {
    xy: Vec2,
    vector: Vec2,
}

impl FlowVector {
    pub fn new(xy: Vec2, magnitude: f32) -> Self {
        Self {
            xy,
            vector: Vec2::new(0.0, magnitude),
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
        self.vector.x.powi(2) + self.vector.y.powi(2)
    }

    pub fn draw(&self) {
        let Vec2 { x: x1, y: y1 } = self.xy;
        let Vec2 { x: x2, y: y2 } = self.xy + self.vector;

        // TODO thickness const?
        draw_line(x1, y1, x2, y2, 2.0, BLACK);
        draw_circle_lines( x2, y2, 2.0, 2.0, BLACK);
    }

    pub fn heading(&self) -> f32 {
        f32::atan2(self.vector.y, self.vector.x).to_degrees()
    }
}

pub type FlowVectorFieldBuilderFn = Box<dyn Fn(&Model) -> Vec<FlowVector>>;

pub fn new_right_hand_curve_flow_vectors(model: &Model) -> Vec<FlowVector> {
    debug!("creating new vector field with a right handed curve");
    let (origin_x, origin_y) = model.get_origin();

    (0..model.grid_height)
        .flat_map(move |column_index| {
            (0..model.grid_width).map(move |row_index| {
                let xy = Vec2::new(
                    (row_index as f32 * model.vector_spacing) + origin_x,
                    (column_index as f32 * model.vector_spacing) + origin_y,
                );

                let mut fv = FlowVector::new(xy, model.vector_magnitude);
                let a = (column_index as f32 / model.grid_height as f32) * PI;
                fv.rotate(a.to_degrees());

                fv
            })
        })
        .collect()
}

pub fn new_simplex_noise_flow_vectors(model: &Model) -> Vec<FlowVector> {
    debug!("creating new vector field from Simplex noise");
    let noise = OpenSimplex::new(model.noise_seed);

    new_noise_flow_vectors(model, &noise)
}

pub fn new_basic_multi_noise_flow_vectors(model: &Model) -> Vec<FlowVector> {
    debug!("creating new vector field from Basic Multi-fractal noise");
    let noise: BasicMulti<OpenSimplex> = BasicMulti::new(model.noise_seed);

    new_noise_flow_vectors(model, &noise)
}

pub fn new_billow_noise_flow_vectors(model: &Model) -> Vec<FlowVector> {
    debug!("creating new vector field from Billow noise");
    let noise: Billow<OpenSimplex> = Billow::new(model.noise_seed);

    new_noise_flow_vectors(model, &noise)
}

pub fn new_terraced_billow_noise_flow_vectors(model: &Model) -> Vec<FlowVector> {
    debug!("creating new vector field from Terraced Billow noise");
    let billow: Billow<OpenSimplex> = Billow::new(model.noise_seed);
    let noise = noise::Terrace::new(&billow)
        .add_control_point(0.0001)
        .add_control_point(0.001)
        .add_control_point(0.01)
        .add_control_point(0.1);

    new_noise_flow_vectors(model, &noise)
}

pub fn new_fbm_noise_flow_vectors(model: &Model) -> Vec<FlowVector> {
    debug!("creating new vector field from FBM noise");
    let noise: Fbm<OpenSimplex> = Fbm::new(model.noise_seed);

    new_noise_flow_vectors(model, &noise)
}

pub fn new_hybrid_multi_noise_flow_vectors(model: &Model) -> Vec<FlowVector> {
    debug!("creating new vector field from Hybrid Multi-fractal noise");
    let noise: HybridMulti<OpenSimplex> = HybridMulti::new(model.noise_seed);

    new_noise_flow_vectors(model, &noise)
}

pub fn new_value_noise_flow_vectors(model: &Model) -> Vec<FlowVector> {
    debug!("creating new vector field from Value noise");
    let noise = noise::Value::new(model.noise_seed);

    new_noise_flow_vectors(model, &noise)
}

pub fn new_worley_noise_flow_vectors(model: &Model) -> Vec<FlowVector> {
    debug!("creating new vector field from Worley (Voronoi-like) noise");
    let noise = noise::Worley::new(model.noise_seed);

    new_noise_flow_vectors(model, &noise)
}

pub fn new_noise_flow_vectors(model: &Model, noise: &dyn NoiseFn<f64, 2>) -> Vec<FlowVector> {
    let (origin_x, origin_y) = model.get_origin();

    (0..model.grid_height)
        .flat_map(move |column_index| {
            (0..model.grid_width).map(move |row_index| {
                let xy = Vec2::new(
                    (row_index as f32 * model.vector_spacing) + origin_x,
                    (column_index as f32 * model.vector_spacing) + origin_y,
                );

                let mut fv = FlowVector::new(xy, model.vector_magnitude);
                let noise_value = noise.get([
                    row_index as f64 * model.noise_scale,
                    column_index as f64 * model.noise_scale,
                ]);
                let a = noise_value as f32 * TAU;
                fv.rotate(a.to_degrees());

                fv
            })
        })
        .collect()
}

pub enum FlowVectorFieldBuilder {
    RightHandCurve,
    BasicMulti,
    Billow,
    TerracedBillow,
    Fbm,
    HybridMulti,
    OpenSimplex,
    Value,
    Worley,
}

impl FlowVectorFieldBuilder {
    pub fn next(&self) -> FlowVectorFieldBuilder {
        match self {
            Self::RightHandCurve => Self::BasicMulti,
            Self::BasicMulti => Self::Billow,
            Self::Billow => Self::TerracedBillow,
            Self::TerracedBillow => Self::Fbm,
            Self::Fbm => Self::HybridMulti,
            Self::HybridMulti => Self::OpenSimplex,
            Self::OpenSimplex => Self::Value,
            Self::Value => Self::Worley,
            Self::Worley => Self::RightHandCurve,
        }
    }

    pub fn as_fn(&self) -> FlowVectorFieldBuilderFn {
        Box::new(match self {
            Self::RightHandCurve => new_right_hand_curve_flow_vectors,
            Self::BasicMulti => new_basic_multi_noise_flow_vectors,
            Self::Billow => new_billow_noise_flow_vectors,
            Self::TerracedBillow => new_terraced_billow_noise_flow_vectors,
            Self::Fbm => new_fbm_noise_flow_vectors,
            Self::HybridMulti => new_hybrid_multi_noise_flow_vectors,
            Self::OpenSimplex => new_simplex_noise_flow_vectors,
            Self::Value => new_value_noise_flow_vectors,
            Self::Worley => new_worley_noise_flow_vectors,
        })
    }
}
