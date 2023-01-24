use glam::{Mat4, Vec3};

use crate::model::Mesh;

#[derive(Debug, Default)]
struct Rotation {
    euler: Vec3, // x: roll, y: pitch, z: yaw
    matrix: Option<Mat4>,
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