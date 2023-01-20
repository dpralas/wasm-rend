use std::collections::HashMap;

use log::info;
use wgpu::{BindGroup, BindGroupLayout, ShaderModule, Texture};
use winit::dpi::PhysicalSize;

pub struct WgpuContext {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    shaders: HashMap<&'static str, ShaderModule>,
    textures: HashMap<&'static str, (Texture, BindGroup, BindGroupLayout)>,
}

impl WgpuContext {
    // Creating some of the wgpu types requires async code
    pub async fn new(canvas: &web_sys::HtmlCanvasElement) -> Self {
        let (width, height) = (canvas.width(), canvas.height());
        info!("Surface size: {}x{}", width, height);

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
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
                    // WebGL doesn't support all of wgpu's features, so if
                    // we disable most features.
                    limits: wgpu::Limits::downlevel_webgl2_defaults(),
                    label: None,
                },
                None, // Trace path
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
            textures: HashMap::new(),
        }
    }
}
