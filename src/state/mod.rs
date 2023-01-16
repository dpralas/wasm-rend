mod internal;

use std::io::Cursor;

use internal::InternalState;
use log::info;
use wasm_bindgen::prelude::*;

use crate::model::mesh::Mesh;

#[wasm_bindgen]
#[derive(Debug)]
pub struct State {
    pub counter: usize,
    #[wasm_bindgen(skip)]
    pub internal: InternalState,
}

#[wasm_bindgen]
impl State {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            counter: 0,
            internal: InternalState { meshes: Vec::new() },
        }
    }
    pub fn add_stl(&mut self, input: &[u8]) {
        let reader = Cursor::new(input);
        let mesh = Mesh::from_stl(reader).unwrap();
        info!("vertices: {:?}", mesh.vertices.len());
        info!("faces: {:?}", mesh.faces.len());
        info!("min: {:?}", mesh.min);
        info!("max: {:?}", mesh.max);
        self.internal.meshes.push(mesh);
    }
    pub fn echo(&self) {
        info!("{:?}", self);
    }
}
