struct VertexOut {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) normal: vec3<f32>,
};

struct FragmentOut {
    @location(0) color: vec4<f32>,
};

@fragment
fn fs_main(fragment_in: VertexOut) -> FragmentOut {
    var fragment_out: FragmentOut;
    let norm = normalize(fragment_in.normal);
    let pos = vec3(fragment_in.clip_position[0], fragment_in.clip_position[1], fragment_in.clip_position[2]);
    let light_dir = normalize(vec3(3.0, 3.0, 2.0) - pos);
    let diff = max(dot(norm, light_dir), 0.0);
    let diffuse = vec4(diff, diff, diff, 1.0) * vec4(1.0, 0.0, 0.0, 1.0);
    fragment_out.color = diffuse;
    return fragment_out;
}
