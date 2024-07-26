#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var<uniform> color: vec4<f32>;
@group(2) @binding(1) var radius_texture: texture_2d<f32>;
@group(2) @binding(2) var radius_sampler: sampler;

const TAU = 6.283185307179586;
const PI = TAU / 2;
const TRANS = vec4<f32>(0.0,0.0,0.0,0.0);
const WHITE = vec4<f32>(1.0,1.0,1.0,1.0);

fn rgb(r:f32,g:f32,b:f32) -> vec4<f32> {
    return vec4<f32>(r,g,b,1.0);
}

// alphablend for colors with pre-multiplied alpha
fn blend(a:vec4<f32>, b:vec4<f32>) -> vec4<f32> {
    return a + b * (1.0 - a.a);
}

fn add_guide(res: vec4<f32>, color: vec4<f32>, guide: f32) -> vec4<f32> {
    if guide < 0.0 {return res;}
    return blend(res, color * min(1.0, guide));
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let pos : vec2<f32> = mesh.uv * 2.0 - vec2<f32>(1.0,1.0);
    let arg = (PI + atan2(pos.x, pos.y)) / TAU;
    let radius = textureSample(radius_texture, radius_sampler, vec2<f32>(arg, 0.0)).r;
    let len = length(pos);
    let pixels = fwidth(pos.x);
    var res: vec4<f32>;

    // Wave generation
    let dist0 = len - 0.5;
    let dist1 = 0.5 + 0.5 * radius - len;
    let dist = min(dist0, dist1);
    let val = dist1 * 4.0;
    if val <= 0.0 {
        res = TRANS;
    } else if val <= 1.0 { 
        res = color * val;
    } else {
        res = mix(color, WHITE, val - 1.0);
    }
    res *= min(0.3, dist0 / pixels);

    // Guide inner
    res = add_guide(res, color, 2.0 - abs(dist1-2.0*pixels) / pixels);
    res = add_guide(res, 1.0*WHITE, 1.0 - abs(len-0.5) / pixels);
    res = add_guide(res, 0.1*WHITE, 1.0 - abs(len-1.0+pixels) / pixels);

    if res.a <= 0.01 { discard; }
    return res;

}
