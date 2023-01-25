use crate::model::{Camera, Mesh};

#[derive(Debug, Clone, Default)]
pub struct Engine {
    pub camera: Camera,
    pub meshes: Vec<Mesh>,
}

impl Engine {
    pub fn add_mesh(&mut self, mesh: Mesh) {
        self.meshes.push(mesh);
    }
}
