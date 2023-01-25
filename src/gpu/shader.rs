use wgpu::{Device, ShaderModule};

pub struct Shader {
    pub name: &'static str,
    pub source: &'static str,
}

impl Shader {
    pub fn bind(&self, device: &Device) -> ShaderModule {
        device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(self.name),
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(
                self.source,
            )),
        })
    }
}
