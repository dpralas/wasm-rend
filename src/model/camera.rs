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
            eye: glam::Vec3::new(1.0, -5.0, 3.0) * 30.0,
            focus: Vec3::ZERO,
            up: Vec3::Z,
            aspect: 1280.0 / 720.0,
            fov_y: core::f32::consts::FRAC_PI_4,
            z_near: 0.01,
            z_far: 10000.0,
        }
    }
}

impl Camera {
    pub fn build_view_projection_matrix(&self) -> Mat4 {
        let projection = glam::Mat4::perspective_rh(
            core::f32::consts::FRAC_PI_4,
            1280.0 / 720.0,
            self.z_near,
            self.z_far,
        );
        let view = glam::Mat4::look_at_rh(self.eye, self.focus, self.up);
        projection * view
    }
}
