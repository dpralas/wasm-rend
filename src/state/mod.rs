mod internal;
mod shared;

use std::io::Cursor;

use internal::InternalState;
use log::info;
use wasm_bindgen::prelude::*;

use crate::{model::mesh::Mesh, state::shared::SharedState};

#[wasm_bindgen]
#[derive(Debug, Clone, Default)]
pub struct State {
    shared: SharedState,
    internal: Option<InternalState>,
}

#[wasm_bindgen]
impl State {
    pub fn add_stl(&mut self, input: &[u8]) {
        let reader = Cursor::new(input);
        let mesh = Mesh::from_stl(reader).unwrap();
        if let Some(ref mut internal_state) = self.internal {
            internal_state.meshes.push(mesh);
        } else {
            self.internal = Some(InternalState { meshes: vec![mesh] });
        }
        info!("{:?}", self);
    }
    pub fn echo(&self) {
        info!("{:?}", self);
    }
}
