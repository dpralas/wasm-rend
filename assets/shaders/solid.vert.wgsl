struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

//struct ModelUniform {
//    transform: mat4x4<f32>,
//};
//@group(1) @binding(0)
//var<uniform> model: ModelUniform;

struct VertexIn {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
}

struct VertexOut {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) normal: vec3<f32>,
};

@vertex
fn vs_main(vertex_in: VertexIn) -> VertexOut {
    var vertex_out: VertexOut;
//    vertex_out.clip_position = camera.view_proj * model.transform * vec4(vertex_in.position, 1.0);
    vertex_out.clip_position = camera.view_proj * vec4(vertex_in.position, 1.0);
    vertex_out.normal = vertex_in.normal;
    return vertex_out;
}
