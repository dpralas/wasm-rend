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
    fragment_out.color = vec4<f32>(1.0, 0.0, 0.0, 1.0);
    return fragment_out;
}
