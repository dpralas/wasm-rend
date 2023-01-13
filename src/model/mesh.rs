use glam::{Mat4, Vec3};
#[cfg(feature = "parallel")]
use rayon::prelude::*;

#[derive(Debug, Clone)]
enum SupportItem {
    Cone,
    Cylinder,
}

#[derive(Default, Debug, Clone)]
enum MeshType {
    #[default]
    Model,
    Support(SupportItem),
}

#[derive(Default, Debug, Clone)]
pub struct Mesh {
    vertices: Vec<Vec3>,
    vertex_normals: Vec<Vec3>,
    face_normals: Vec<Vec3>,
    faces: Vec<[usize; 3]>,
    convex_hull: Option<Box<Mesh>>,
    max: Vec3,
    min: Vec3,
    settle_transform: Option<(Mat4, Vec3)>,
    mesh_type: MeshType,
}
