use wgpu::{util::DeviceExt, BindGroup, BindGroupLayout, Buffer, Device};

use super::{index::GpuIndex, BoundBuffer, GpuBuffer};
use crate::{
    gpu::types::vertex::GpuVertex,
    model::{Mesh, Object},
};

#[derive(Copy, Clone, Debug)]
pub struct GpuObject<'a> {
    mesh: &'a Mesh,
    object: &'a Object,
}

impl<'a> From<(&'a Mesh, &'a Object)> for GpuObject<'a> {
    fn from((mesh, object): (&'a Mesh, &'a Object)) -> Self {
        Self { mesh, object }
    }
}

impl<'a> GpuBuffer for GpuObject<'a> {
    fn bind(&self, device: &Device) -> BoundBuffer {
        let layout = self.create_bind_group_layout(device);
        let content = self.get_vertex_buffer_contents();
        let buffer = self.create_vertex_buffer(device, &content);
        let group = self.create_bind_group(&buffer, &layout, device);
        BoundBuffer {
            buffer,
            content,
            group,
            layout,
        }
    }
}

impl<'a> GpuObject<'a> {
    fn get_vertex_buffer_contents(&self) -> Vec<u8> {
        let contents = self
            .mesh
            .vertices
            .iter()
            .zip(self.mesh.vertex_normals.iter())
            .map(|(vertex, vertex_normal)| (vertex, vertex_normal).into())
            .collect::<Vec<GpuVertex>>();
        bytemuck::cast_slice(&contents[..]).to_vec()
    }

    fn get_index_buffer_contents(&self) -> Vec<u8> {
        let contents = &self
            .mesh
            .indices
            .iter()
            .map(|index| index.into())
            .collect::<Vec<GpuIndex>>();
        bytemuck::cast_slice(&contents[..]).to_vec()
    }

    fn create_vertex_buffer(
        &self,
        device: &Device,
        buffer_contents: &[u8],
    ) -> Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!(
                "Object {:?} Vertex Buffer",
                self.object.name
            )),
            contents: buffer_contents,
            usage: wgpu::BufferUsages::VERTEX,
        })
    }

    fn create_index_buffer(
        &self,
        device: &Device,
        buffer_contents: &[u8],
    ) -> Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("Object {:?} Index Buffer", self.object.name)),
            contents: buffer_contents,
            usage: wgpu::BufferUsages::INDEX,
        })
    }

    fn create_bind_group_layout(&self, device: &Device) -> BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX
                    | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some(&format!(
                "Object {:?} Bind Group Layout",
                self.object.name
            )),
        })
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
            label: Some(&format!("Object {:?} Bind Group", self.object.name)),
        })
    }
}
