//! Camera implementation based on following article:
//! https://sotrh.github.io/learn-wgpu/beginner/tutorial6-uniforms/#a-perspective-camera

use glam::{Mat4, Vec3};

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Mat4 = Mat4::from_cols_array(&[
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
]);

#[derive(Debug, Clone)]
pub struct Camera {
    eye: Vec3,
    focus: Vec3,
    up: Vec3,
    aspect: f32,
    fov_y: f32,
    z_near: f32,
    z_far: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            eye: Vec3::new(0.0, 1.0, 2.0),
            focus: Vec3::ZERO,
            up: Vec3::Y,
            aspect: 640.0 / 480.0,
            fov_y: 45.0,
            z_near: 0.1,
            z_far: 100.0,
        }
    }
}

impl Camera {
    fn build_view_projection_matrix(&self) -> Mat4 {
        let view = Mat4::look_at_rh(self.eye, self.focus, self.up);
        let proj = Mat4::perspective_lh(
            f32::to_radians(self.fov_y),
            self.aspect,
            self.z_near,
            self.z_far,
        );

        OPENGL_TO_WGPU_MATRIX * proj * view
    }
}
