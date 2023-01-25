use glam::{Mat4, Vec3};

use crate::model::Mesh;

#[derive(Debug, Default)]
struct Rotation {
    euler: Vec3, // x: roll, y: pitch, z: yaw
    matrix: Mat4,
}

#[derive(Debug, Default)]
pub struct Transform {
    translation: Vec3,
    scale: Vec3,
    rotation: Rotation,
}

#[derive(Debug)]
pub struct Object<'mesh> {
    pub mesh: &'mesh Mesh,
    pub transform: Transform,
}

impl<'mesh> From<&'mesh Mesh> for Object<'mesh> {
    fn from(mesh: &'mesh Mesh) -> Self {
        Self {
            mesh: mesh,
            transform: Transform::default(),
        }
    }
}

impl<'mesh> Object<'mesh> {
    pub fn get_transform_matrix(&self) -> Mat4 {
        Mat4::from_translation(self.transform.translation)
            * self.transform.rotation.matrix
            * Mat4::from_scale(self.transform.scale)
    }
}
