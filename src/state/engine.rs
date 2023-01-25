use crate::model::{Camera, Mesh, Object};

#[derive(Debug, Clone, Default)]
pub struct Engine {
    pub camera: Camera,
    pub meshes: Vec<Mesh>,
    pub objects: Vec<Object>,
    objects_loaded: usize,
}

impl Engine {
    pub fn add_mesh(&mut self, mesh: Mesh) {
        self.meshes.push(mesh);
        self.objects
            .push(Object::with_name(&self.objects_loaded.to_string()));
        self.objects_loaded += 1;
    }
}
