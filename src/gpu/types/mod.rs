pub mod camera;
pub mod index;
pub mod object;
pub mod texture;
pub mod vertex;

use wgpu::{BindGroup, BindGroupLayout, Buffer, Device, VertexBufferLayout};

pub struct BoundBuffer {
    pub buffer: Buffer,
    pub content: Vec<u8>,
    pub group: BindGroup,
    pub layout: BindGroupLayout,
}

pub trait GpuPrimitive {
    fn data(&self) -> Vec<u8>;
    fn desc<'a>() -> VertexBufferLayout<'a>;
}

pub trait GpuBuffer {
    fn bind(&self, device: &Device) -> BoundBuffer;
}
