use wgpu::VertexBufferLayout;

use crate::gpu::GpuPrimitive;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct GpuIndex([u32; 3]);

unsafe impl bytemuck::Pod for GpuIndex {}
unsafe impl bytemuck::Zeroable for GpuIndex {}

impl GpuIndex {
    pub const BUFFER_LAYOUT: VertexBufferLayout<'static> =
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<GpuIndex>()
                as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Uint32x3],
        };
}

impl GpuPrimitive for GpuIndex {
    fn data(&self) -> Vec<u8> {
        bytemuck::cast_slice(&[*self]).to_vec()
    }

    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        Self::BUFFER_LAYOUT
    }
}

impl From<&[usize; 3]> for GpuIndex {
    fn from(index: &[usize; 3]) -> Self {
        GpuIndex([index[0] as u32, index[1] as u32, index[2] as u32])
    }
}
