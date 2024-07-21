#import bevy_sprite::mesh2d_vertex_output::VertexOutput

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
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let pos : vec2<f32> = mesh.uv * 2.0 - vec2<f32>(1.0,1.0);
    let dist = length(pos);
    let aa = fwidth(dist);
    let edge0 = dist - 1.0 + width;
    let edge1 = 1.0 - dist;
    if 0.0 < edge0 && 0.0 < edge1 {
        let minedge = min(edge0, edge1);
        if minedge < aa {
            return vec4<f32>(color.rgb, minedge / aa);
        }
        return color;
    } 
    discard;
}
