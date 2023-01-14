use std::io::Cursor;

use log::info;
use wasm_bindgen::prelude::*;

use crate::model::mesh::Mesh;

#[wasm_bindgen]
#[derive(Debug)]
pub struct SharedState {
    counter: usize,
}

#[wasm_bindgen]
impl SharedState {
    pub fn new() -> Self {
        Self { counter: 0 }
    }
    pub fn update(&mut self, new: SharedState) {
        *self = new;
        info!("state: {:?}", self);
    }
    pub fn display(&self) -> String {
        info!("state: {:?}", self);
        format!("{:?}", self)
    }
    pub fn echo(input: &str) {
        info!("{:?}", input);
    }
    pub fn add_stl(input: &[u8]) {
        let reader = Cursor::new(input);
        let mesh = Mesh::from_stl(reader).unwrap();
        info!("vertices: {:?}", mesh.vertices.len());
        info!("faces: {:?}", mesh.faces.len());
        info!("min: {:?}", mesh.min);
        info!("max: {:?}", mesh.max);
    }
}
