mod internal;
mod shared;

use std::io::Cursor;

use internal::InternalState;
use log::info;
use wasm_bindgen::prelude::*;

use crate::{model::Mesh, state::shared::SharedState};

#[wasm_bindgen]
#[derive(Debug, Clone, Default)]
pub struct State {
    shared: SharedState,
    internal: InternalState,
}

#[wasm_bindgen]
impl State {
    pub fn add_stl(&mut self, input: &[u8]) {
        let reader = Cursor::new(input);
        let mesh = Mesh::from_stl(reader).unwrap();
        info!("{:?}", self);
    }
    pub fn echo(&self) {
        info!("{:?}", self);
    }
}
