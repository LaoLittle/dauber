struct VertexOutput {
    //@location(0) v_color: vec4<f32>,
    @builtin(position) position: vec4<f32>,
};

struct VertexInput {
    @location(0) pos: vec2<f32>,
    //@builtin(vertex_index) vindex: u32,
    //@builtin(instance_index) instance_idx: u32
};

struct Globals {
    view: vec2<f32>,
};

struct Vertex {
    pos: vec2<f32>,
};

@group(0)
@binding(0)
var<uniform> globals: Globals;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    let invert_y = vec2<f32>(1.0, -1.0);

    return VertexOutput(
        vec4<f32>(((in.pos / globals.view) * 2.0 - 1.0) * invert_y, 0.0, 1.0)
    );
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
// rgba
    return vec4<f32>(0.0, 1.0, 1.0, 1.0);
}