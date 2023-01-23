use wgpu::{util::DeviceExt, BindGroup, BindGroupLayout, Buffer, Device};

use super::index::GpuIndex;
use crate::{
    gpu::{types::vertex::GpuVertex, GpuPrimitive},
    model::Object,
};

#[derive(Copy, Clone, Debug)]
pub struct GpuObject<'a> {
    object: &'a Object<'a>,
}

impl<'a> From<&'a Object<'a>> for GpuObject<'a> {
    fn from(object: &'a Object) -> Self {
        Self { object }
    }
}

impl<'a> GpuObject<'a> {
    fn get_vertices_buffer_contents(&self) -> Vec<u8> {
        let contents = self
            .object
            .mesh
            .vertices
            .iter()
            .map(|vertex| vertex.into())
            .collect::<Vec<GpuVertex>>();
        bytemuck::cast_slice(&contents[..]).to_vec()
    }

    fn get_indices_buffer_contents(&self) -> Vec<u8> {
        let contents = &self
            .object
            .mesh
            .indices
            .iter()
            .map(|index| index.into())
            .collect::<Vec<GpuIndex>>();
        bytemuck::cast_slice(&contents[..]).to_vec()
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
            label: Some("Object Bind Group"),
        })
    }

    fn create_buffer(&self, device: &Device, buffer_contents: &[u8]) -> Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Object Buffer"),
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
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("Object Bind Group Layout"),
        })
    }
}
