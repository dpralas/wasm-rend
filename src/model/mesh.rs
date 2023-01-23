use glam::{Mat4, Vec3};
#[cfg(feature = "parallel")]
use rayon::prelude::*;

#[derive(Debug, Clone)]
pub enum SupportItem {
    Cone,
    Cylinder,
}

#[derive(Default, Debug, Clone)]
pub enum MeshType {
    #[default]
    Model,
    Support(SupportItem),
}

#[derive(Default, Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub indices: Vec<[usize; 3]>,
    pub vertex_normals: Vec<Vec3>,
    pub index_normals: Vec<Vec3>,
    pub convex_hull: Option<Box<Mesh>>,
    pub max: Vec3,
    pub min: Vec3,
    pub settle_transform: Option<(Mat4, Vec3)>,
    pub mesh_type: MeshType,
}

impl Mesh {
    fn calculate_min(&self) -> Vec3 {
        #[cfg(not(feature = "parallel"))]
        let iter = self.vertices.iter();
        #[cfg(feature = "parallel")]
        let iter = self.vertices.par_iter();

        let iter = iter.cloned();

        #[cfg(not(feature = "parallel"))]
        let min = iter.fold(Vec3::splat(std::f32::MAX), Vec3::min);
        #[cfg(feature = "parallel")]
        let min = iter.reduce(|| Vec3::splat(std::f32::MAX), Vec3::min);
        min
    }

    fn calculate_max(&self) -> Vec3 {
        #[cfg(not(feature = "parallel"))]
        let iter = self.vertices.iter();
        #[cfg(feature = "parallel")]
        let iter = self.vertices.par_iter();

        let iter = iter.cloned();

        #[cfg(not(feature = "parallel"))]
        let max = iter.fold(Vec3::splat(std::f32::MIN), Vec3::max);
        #[cfg(feature = "parallel")]
        let max = iter.reduce(|| Vec3::splat(std::f32::MIN), Vec3::max);
        max
    }

    pub fn recalculate_min_and_max(&mut self) {
        self.min = self.calculate_min();
        self.max = self.calculate_max();
    }
}
