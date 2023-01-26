use std::collections::HashMap;

use log::info;
use wgpu::{Color, ShaderModule};
use winit::dpi::PhysicalSize;

use super::{frame::Frame, pipeline, Shader};
use crate::state::State;

pub struct WgpuContext {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    shaders: HashMap<&'static str, ShaderModule>,
}

impl WgpuContext {
    pub async fn new(canvas: &web_sys::HtmlCanvasElement) -> Self {
        let (width, height) = (canvas.width(), canvas.height());
        info!("Surface size: {}x{}", width, height);
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = instance.create_surface_from_canvas(canvas);

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let backend = format!("{:?}", adapter.get_info().backend);
        info!("Backend: {}", backend);

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::downlevel_webgl2_defaults(),
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width,
            height,
            present_mode: wgpu::PresentMode::AutoNoVsync,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };
        surface.configure(&device, &config);

        Self {
            surface,
            device,
            queue,
            config,
            size: PhysicalSize::new(width, height),
            shaders: HashMap::new(),
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn add_shader(&mut self, name: &'static str, source: &'static str) {
        if self.shaders.contains_key(name) {
            panic!("Shader with name '{}' already exists", name);
        }
        let shader = Shader { name, source };
        self.shaders.insert(name, shader.bind(&self.device));
    }

    pub fn get_shader(&self, name: &'static str) -> &ShaderModule {
        self.shaders
            .get(name)
            .unwrap_or_else(|| panic!("No shader with name '{}'", name))
    }

    pub fn render(&mut self, state: &State) -> Result<(), wgpu::SurfaceError> {
        info!("render called");
        // Get the surface texture we will draw on
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // Encoder will send commands to the queue
        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            },
        );

        let frame = Frame::from(&state.engine);
        let camera_uniform = frame.create_camera_binding(&self.device);
        let pipeline = {
            let pipeline_layout = self.device.create_pipeline_layout(
                &wgpu::PipelineLayoutDescriptor {
                    label: Some("Solid Pipeline Layout"),
                    bind_group_layouts: &[&camera_uniform.layout],
                    push_constant_ranges: &[],
                },
            );
            pipeline::get(self, pipeline_layout)
        };

        let vertex_buffers = frame.create_vertex_buffers(&self.device);
        let index_buffers = frame.create_index_buffers(&self.device);

        // Execute render pass
        {
            // Make pass
            let mut pass =
                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(
                        wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(Color {
                                    r: 0.1,
                                    g: 0.2,
                                    b: 0.3,
                                    a: 1.0,
                                }),
                                store: true,
                            },
                        },
                    )],
                    depth_stencil_attachment: None,
                });

            // Draw world data
            pass.set_pipeline(&pipeline);
            pass.set_bind_group(0, &camera_uniform.group, &[]);
            for (n, (vertex_buffer, index_buffer)) in
                vertex_buffers.iter().zip(index_buffers.iter()).enumerate()
            {
                pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                pass.set_index_buffer(
                    index_buffer.slice(..),
                    wgpu::IndexFormat::Uint32,
                );
                let num_indexes = state.engine.meshes[n].indices.len();
                info!("indexes: {num_indexes}");
                pass.draw_indexed(0..num_indexes as u32, 0, 0..1)
            }
        }

        // Write buffers
        self.queue.write_buffer(
            &camera_uniform.buffer,
            0,
            &camera_uniform.content,
        );

        // Submit queue
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
