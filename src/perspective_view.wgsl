struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] color: vec3<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] color: vec3<f32>;
};

[[stage(vertex)]]
fn vs_main(
    in: VertexInput,
) -> VertexOutput {

    const a = (umax - umin) / (xmax - xmin);
    const b = (vmax - vmin) / (ymax - ymin);
    const c = -xmin * a + umin;
    const d = -ymin * b + vmin;

    const mjp = mat4x4<f32>(
        vec4<f32>(  a, 0.0, 0.0, 0.0),
        vec4<f32>(0.0,   b, 0.0, 0.0),
        vec4<f32>(0.0, 0.0, 1.0, 0.0),
        vec4<f32>(  c,   d, 0.0, 1.0),
    );


    


    var out: VertexOutput;
    out.color = in.color;
    out.clip_position = vec4<f32>(in.position[0], in.position[1], 0.0, 1.0); // front view
    return out;
}

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
