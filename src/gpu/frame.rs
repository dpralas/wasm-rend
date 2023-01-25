use wgpu::{util::DeviceExt, Device};

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
}
