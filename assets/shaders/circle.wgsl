@vertex
fn vertex(
    @location(0) position: vec3<f32>,
    @builtin(instance_index) instance_index: u32
) -> @builtin(position) vec4<f32> {
    return vec4<f32>(position, 1.0);
}

@fragment
fn fragment() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0); // Red color
}
