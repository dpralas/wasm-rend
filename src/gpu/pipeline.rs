use wgpu::{DepthStencilState, Face, PipelineLayout};

use super::{GpuPrimitive, GpuVertex};

pub(crate) fn get(
    context: &crate::gpu::WgpuContext,
    layout: PipelineLayout,
) -> wgpu::RenderPipeline {
    let solid_vert = context.get_shader("solid.vert");
    let solid_frag = context.get_shader("solid.frag");

    let pipeline = context.device.create_render_pipeline(
        &wgpu::RenderPipelineDescriptor {
            label: Some("Solid Pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: solid_vert,
                entry_point: "vs_main",
                buffers: &[GpuVertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: solid_frag,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: context.config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                // Setting this to anything other than Fill requires
                // Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: Some(DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        },
    );

    pipeline
}
