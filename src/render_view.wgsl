@group(0) @binding(0)
var texture: texture_2d<f32>;

@group(0) @binding(1)
var texture_sampler: sampler;


struct RenderViewUniform {
    screen_resolution: vec2<f32>,
}

@group(0) @binding(2)
var<uniform> render_view_uniform: RenderViewUniform;

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
    let vert_pos_2d = vec2<f32>(f32((in_vertex_index << 1) & 2), f32(in_vertex_index & 2));
    return vec4<f32>(vert_pos_2d * 2.0 - 1.0, 0.0, 1.0);;
}



@fragment
fn fs_main(@builtin(position) coord: vec4<f32>) -> @location(0) vec4<f32> {
    var uv_coord = coord.xy / render_view_uniform.screen_resolution;

    return textureSample(texture, texture_sampler, uv_coord);
}