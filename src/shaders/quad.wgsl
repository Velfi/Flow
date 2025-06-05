@group(0) @binding(0)
var background_texture: texture_2d<f32>;
@group(0) @binding(1)
var background_sampler: sampler;

@group(1) @binding(0)
var vector_texture: texture_2d<f32>;
@group(1) @binding(1)
var vector_sampler: sampler;

@group(2) @binding(0)
var particle_texture: texture_2d<f32>;
@group(2) @binding(1)
var particle_sampler: sampler;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var pos = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(3.0, -1.0),
        vec2<f32>(-1.0, 3.0)
    );

    var tex_coords = array<vec2<f32>, 3>(
        vec2<f32>(0.0, 1.0),
        vec2<f32>(2.0, 1.0),
        vec2<f32>(0.0, -1.0)
    );

    var output: VertexOutput;
    output.position = vec4<f32>(pos[vertex_index], 0.0, 1.0);
    output.tex_coords = tex_coords[vertex_index];
    return output;
}

fn over(a: vec4<f32>, b: vec4<f32>) -> vec4<f32> {
    // a is the top layer, b is the bottom layer
    return a + b * (1.0 - a.a);
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Sample all layers
    let background = textureSample(background_texture, background_sampler, input.tex_coords);
    let vector = textureSample(vector_texture, vector_sampler, input.tex_coords);
    let particle = textureSample(particle_texture, particle_sampler, input.tex_coords);

    // Composite layers with correct alpha blending: background -> vector -> particle
    let color = over(particle, over(vector, background));
    return color;
}