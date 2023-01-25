use glam::{Mat4, Vec3};

#[derive(Debug, Clone, Default)]
struct Rotation {
    euler: Vec3, // x: roll, y: pitch, z: yaw
    matrix: Mat4,
}

#[derive(Debug, Clone, Default)]
pub struct Transform {
    translation: Vec3,
    scale: Vec3,
    rotation: Rotation,
}

#[derive(Debug, Clone, Default)]
pub struct Object {
    pub name: Option<String>,
    pub transform: Transform,
}

impl Object {
    pub fn with_name(name: &str) -> Self {
        Self {
            name: Some(name.to_owned()),
            transform: Transform::default(),
        }
    }

    pub fn get_transform_matrix(&self) -> Mat4 {
        Mat4::from_translation(self.transform.translation)
            * self.transform.rotation.matrix
            * Mat4::from_scale(self.transform.scale)
    }
}
