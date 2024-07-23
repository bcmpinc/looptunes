#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var<uniform> color: vec4<f32>;
@group(2) @binding(1) var radius_texture: texture_2d<f32>;
@group(2) @binding(2) var radius_sampler: sampler;

const TAU = 6.283185307179586;
const PI = TAU / 2;
const BLACK = vec4<f32>(0.0,0.0,0.0,1.0);
const GRAY  = vec4<f32>(0.1,0.1,0.1,1.0);
const WHITE = vec4<f32>(1.0,1.0,1.0,1.0);

fn with_alpha(c:vec4<f32>, a: f32) -> vec4<f32> {
    return vec4<f32>(c.rgb, a);
}
fn to_pre(c:vec4<f32>) -> vec4<f32> {
    return vec4<f32>(c.rgb * c.a, c.a);
}
fn to_straight(c:vec4<f32>) -> vec4<f32> {
    return vec4<f32>(c.rgb / c.a, c.a);
}
// alphablend for colors with pre-multiplied alpha
fn blend(a:vec4<f32>, b:vec4<f32>) -> vec4<f32>{
    return a + b * (1.0 - a.a);
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let pos : vec2<f32> = mesh.uv * 2.0 - vec2<f32>(1.0,1.0);
    let arg = (PI + atan2(pos.x, pos.y)) / TAU;
    let radius = textureSample(radius_texture, radius_sampler, vec2<f32>(arg, 0.0)).r;
    let dist = (0.9 - length(pos)) / 0.1;
    let guide = 15.0-abs(dist) / dpdx(pos.x);
    let pixels = 2.0*(radius - dist * dist);
    if pixels <= 0.0 && guide <= 0.0 { discard; }
    if pixels <= 1.0 { 
        return to_straight(blend(color * pixels, WHITE * 0.1 * clamp(guide,0.0,1.0)));
    }
    return mix(color, WHITE, pixels-1.0);
}
