use crate::model::Model;
use log::debug;
use glam::Vec2;
use noise::{NoiseFn, OpenSimplex, Billow, BasicMulti, Fbm, HybridMulti, Value, Worley, MultiFractal};

const TAU: f32 = 2.0 * std::f32::consts::PI;

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
        self.mag_sq().sqrt()
    }

    fn mag_sq(&self) -> f32 {
        self.vector.x.powi(2) + self.vector.y.powi(2)
    }

    pub fn heading(&self) -> f32 {
        self.vector.y.atan2(self.vector.x).to_degrees()
    }

    // Add getter methods for private fields
    pub fn position(&self) -> Vec2 {
        self.xy
    }

    pub fn direction(&self) -> Vec2 {
        self.vector
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
                let a = (column_index as f32 / model.grid_height as f32) * std::f32::consts::PI;
                fv.rotate(a.to_degrees());
                fv
            })
        })
        .collect()
}

// Generic noise function that works with any NoiseFn
fn create_noise_flow_vectors<N: NoiseFn<f64, 2> + Clone>(model: &Model, noise: N) -> Vec<FlowVector> {
    let (origin_x, origin_y) = model.get_origin();
    
    // Generate random offsets for X and Y
    let mut rng = rand::thread_rng();
    let x_offset = rand::Rng::gen_range(&mut rng, 0.0..1000.0);
    let y_offset = rand::Rng::gen_range(&mut rng, 0.0..1000.0);
    
    (0..model.grid_height)
        .flat_map(move |column_index| {
            let noise = noise.clone();
            (0..model.grid_width).map(move |row_index| {
                let xy = Vec2::new(
                    (row_index as f32 * model.vector_spacing) + origin_x,
                    (column_index as f32 * model.vector_spacing) + origin_y,
                );
                let mut fv = FlowVector::new(xy, model.vector_magnitude);
                let noise_value = noise.get([
                    (row_index as f64 * model.noise_scale) + x_offset,
                    (column_index as f64 * model.noise_scale) + y_offset,
                ]) as f32;
                let a = noise_value * TAU;
                fv.rotate(a.to_degrees());
                fv
            })
        })
        .collect()
}

pub fn new_simplex_noise_flow_vectors(model: &Model) -> Vec<FlowVector> {
    debug!("creating new vector field from Simplex noise");
    let noise = OpenSimplex::new(model.noise_seed);
    create_noise_flow_vectors(model, noise)
}

pub fn new_basic_multi_noise_flow_vectors(model: &Model) -> Vec<FlowVector> {
    debug!("creating new vector field from Basic Multi-fractal noise");
    let noise = BasicMulti::<OpenSimplex>::new(model.noise_seed);
    create_noise_flow_vectors(model, noise)
}

pub fn new_billow_noise_flow_vectors(model: &Model) -> Vec<FlowVector> {
    debug!("creating new vector field from Billow noise");
    let noise = Billow::<OpenSimplex>::new(model.noise_seed);
    create_noise_flow_vectors(model, noise)
}

pub fn new_terraced_billow_noise_flow_vectors(model: &Model) -> Vec<FlowVector> {
    debug!("creating new vector field from Terraced Billow noise");
    let mut noise = Billow::<OpenSimplex>::new(model.noise_seed);
    noise = noise.set_octaves(6);
    create_noise_flow_vectors(model, noise)
}

pub fn new_fbm_noise_flow_vectors(model: &Model) -> Vec<FlowVector> {
    debug!("creating new vector field from FBM noise");
    let noise = Fbm::<OpenSimplex>::new(model.noise_seed);
    create_noise_flow_vectors(model, noise)
}

pub fn new_hybrid_multi_noise_flow_vectors(model: &Model) -> Vec<FlowVector> {
    debug!("creating new vector field from Hybrid Multi-fractal noise");
    let noise = HybridMulti::<OpenSimplex>::new(model.noise_seed);
    create_noise_flow_vectors(model, noise)
}

pub fn new_value_noise_flow_vectors(model: &Model) -> Vec<FlowVector> {
    debug!("creating new vector field from Value noise");
    let noise = Value::new(model.noise_seed);
    create_noise_flow_vectors(model, noise)
}

pub fn new_worley_noise_flow_vectors(model: &Model) -> Vec<FlowVector> {
    debug!("creating new vector field from Worley (Voronoi-like) noise");
    let noise = Worley::new(model.noise_seed);
    create_noise_flow_vectors(model, noise)
}

#[derive(Debug, Clone)]
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
