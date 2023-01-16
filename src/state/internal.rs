use crate::model::mesh::Mesh;

#[derive(Debug, Clone)]
pub struct InternalState {
    pub meshes: Vec<Mesh>,
}
