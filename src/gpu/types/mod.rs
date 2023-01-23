mod camera;

use wgpu::{BindGroup, BindGroupLayout, Buffer, Device, VertexBufferLayout};

pub struct BoundUniform {
    buffer: Buffer,
    content: Vec<u8>,
    group: BindGroup,
    layout: BindGroupLayout,
}

pub trait GpuPrimitive {
    fn data(&self) -> Vec<u8>;
    fn desc<'a>() -> VertexBufferLayout<'a>;
}

pub trait GpuUniform {
    fn bind(&self, device: &Device) -> BoundUniform;
}
