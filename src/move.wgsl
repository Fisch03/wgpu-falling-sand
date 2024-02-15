@group(0) @binding(0)
var<storage, read_write> world_view: array<u32>;

struct RenderViewUniform {
    screen_resolution: vec2<f32>,
    view_size: vec2<f32>,
}

@group(0) @binding(1)
var<uniform> render_view_uniform: RenderViewUniform;

fn coord_to_index(coord: vec2<u32>) -> u32 {
    return u32(coord.x + coord.y * u32(render_view_uniform.view_size.x));
}

@compute
@workgroup_size(8,8)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>, @builtin(local_invocation_index) local_index: u32) {
    if(f32(global_id.x) >= render_view_uniform.view_size.x || f32(global_id.y) >= render_view_uniform.view_size.y) {
        return;
    }

    var index = coord_to_index(global_id.xy);

    world_view[index] =  local_index;
}