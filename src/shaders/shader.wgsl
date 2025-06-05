struct Uniforms {
    resolution: vec2<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(@location(0) position: vec2<f32>,
           @location(1) color: vec4<f32>) -> VertexOutput {
    var output: VertexOutput;
    
    // Convert from centered coordinates to normalized device coordinates
    // Input: (-width/2, -height/2) to (width/2, height/2) with (0,0) at center
    // NDC: (-1, -1) to (1, 1) with (0,0) at center
    let ndc_x = position.x / (uniforms.resolution.x * 0.5);
    let ndc_y = position.y / (uniforms.resolution.y * 0.5);
    
    output.position = vec4<f32>(ndc_x, ndc_y, 0.0, 1.0);
    output.color = color;
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return input.color;
} 