use wgpu::{util::DeviceExt, Buffer, Device};

use super::{BoundBuffer, CameraUniform, GpuBuffer, GpuObject};
use crate::state::Engine;

pub struct Frame<'a> {
    camera: CameraUniform<'a>,
    objects: Vec<GpuObject<'a>>,
}

impl<'a> From<&'a Engine> for Frame<'a> {
    fn from(engine: &'a Engine) -> Self {
        Self {
            camera: CameraUniform::from(&engine.camera),
            objects: engine
                .meshes
                .iter()
                .zip(engine.objects.iter())
                .map(|(mesh, object)| GpuObject::from((mesh, object)))
                .collect(),
        }
    }
}

impl<'a> Frame<'a> {
    pub fn create_camera_binding(&self, device: &Device) -> BoundBuffer {
        self.camera.bind(device)
    }

    pub fn create_vertex_buffers(&self, device: &Device) -> Vec<Buffer> {
        self.objects
            .iter()
            .map(|gpu_object| {
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(&format!(
                        "Object {:?} Vertex Buffer",
                        gpu_object.object.name
                    )),
                    contents: &gpu_object.get_vertex_buffer_contents(),
                    usage: wgpu::BufferUsages::VERTEX,
                })
            })
            .collect()
    }

    pub fn create_index_buffers(&self, device: &Device) -> Vec<Buffer> {
        self.objects
            .iter()
            .map(|gpu_object| {
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(&format!(
                        "Object {:?} Index Buffer",
                        gpu_object.object.name
                    )),
                    contents: &gpu_object.get_index_buffer_contents(),
                    usage: wgpu::BufferUsages::INDEX,
                })
            })
            .collect()
    }
}
