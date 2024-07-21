@group(2) @binding(0) var<uniform> color: vec4<f32>;
@group(2) @binding(1) var<uniform> width: f32;

@vertex
fn vertex(
    @location(0) position: vec3<f32>,
    @builtin(instance_index) instance_index: u32
) -> @builtin(position) vec4<f32> {
    return vec4<f32>(position, 1.0);
}

@fragment
fn fragment() -> @location(0) vec4<f32> {
    return color;
}
