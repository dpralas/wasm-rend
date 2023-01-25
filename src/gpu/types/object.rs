use wgpu::{util::DeviceExt, Buffer, Device};

use super::{index::GpuIndex, GpuPrimitive};
use crate::{
    gpu::types::vertex::GpuVertex,
    model::{Mesh, Object},
};

#[derive(Copy, Clone, Debug)]
pub struct GpuObject<'a> {
    mesh: &'a Mesh,
    pub object: &'a Object,
}

impl<'a> From<(&'a Mesh, &'a Object)> for GpuObject<'a> {
    fn from((mesh, object): (&'a Mesh, &'a Object)) -> Self {
        Self { mesh, object }
    }
}

impl<'a> GpuPrimitive for GpuObject<'a> {
    fn data(&self) -> Vec<u8> {
        self.get_vertex_buffer_contents()
    }

    fn desc<'b>() -> wgpu::VertexBufferLayout<'b> {
        GpuVertex::desc()
    }
}

impl<'a> GpuObject<'a> {
    pub fn get_vertex_buffer_contents(&self) -> Vec<u8> {
        let contents = self
            .mesh
            .vertices
            .iter()
            .zip(self.mesh.vertex_normals.iter())
            .map(|(vertex, vertex_normal)| (vertex, vertex_normal).into())
            .collect::<Vec<GpuVertex>>();
        bytemuck::cast_slice(&contents[..]).to_vec()
    }

    pub fn get_index_buffer_contents(&self) -> Vec<u8> {
        let contents = &self
            .mesh
            .indices
            .iter()
            .map(|index| index.into())
            .collect::<Vec<GpuIndex>>();
        bytemuck::cast_slice(&contents[..]).to_vec()
    }

    pub fn create_vertex_buffer(
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

    pub fn create_index_buffer(
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
}
