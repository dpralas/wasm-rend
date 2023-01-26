use wgpu::{util::DeviceExt, BindGroup, BindGroupLayout, Buffer, Device};

use crate::{
    gpu::{BoundBuffer, GpuBuffer},
    model::Camera,
};

pub struct CameraUniform<'a> {
    pub camera: &'a Camera,
}

impl<'a> From<&'a Camera> for CameraUniform<'a> {
    fn from(c: &'a Camera) -> Self {
        CameraUniform { camera: c }
    }
}

impl<'a> GpuBuffer for CameraUniform<'a> {
    fn bind(&self, device: &Device) -> BoundBuffer {
        let layout = self.create_bind_group_layout(device);
        let content = self.get_buffer_contents();
        let buffer = self.create_buffer(device, &content);
        let group = self.create_bind_group(&buffer, &layout, device);
        BoundBuffer {
            buffer,
            content,
            group,
            layout,
        }
    }
}

impl<'a> CameraUniform<'a> {
    fn get_buffer_contents(&self) -> Vec<u8> {
        let mat = self.camera.build_view_projection_matrix();
        let mat_ref = mat.as_ref();
        bytemuck::cast_slice(mat_ref).to_vec()
    }

    fn create_bind_group(
        &self,
        buffer: &Buffer,
        layout: &BindGroupLayout,
        device: &Device,
    ) -> BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some("Camera Bind Group"),
        })
    }

    fn create_buffer(&self, device: &Device, buffer_contents: &[u8]) -> Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: buffer_contents,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        })
    }

    fn create_bind_group_layout(&self, device: &Device) -> BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(64),
                },
                count: None,
            }],
            label: Some("Camera Bind Group Layout"),
        })
    }
}
