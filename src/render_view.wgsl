@group(0) @binding(0)
var<storage, read> world_view: array<u32>;

struct RenderViewUniform {
    screen_resolution: vec2<f32>,
    view_size: vec2<f32>,
}

@group(0) @binding(1)
var<uniform> render_view_uniform: RenderViewUniform;

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
    let vert_pos_2d = vec2<f32>(f32((in_vertex_index << 1) & 2), f32(in_vertex_index & 2));
    return vec4<f32>(vert_pos_2d * 2.0 - 1.0, 0.0, 1.0);;
}

fn coord_to_index(coord: vec2<u32>) -> u32 {
    return u32(coord.x + coord.y * u32(render_view_uniform.view_size.x));
}

@fragment
fn fs_main(@builtin(position) coord: vec4<f32>) -> @location(0) vec4<f32> {
    var view_coord = vec2<u32>(coord.xy / render_view_uniform.screen_resolution * render_view_uniform.view_size);

    var index = coord_to_index(view_coord);


    var element_type = world_view[index];

    return vec4<f32>(f32(element_type)/64.0, 0.0, 0.0, 1.0);
}