use log::info;
use wasm_bindgen::prelude::*;

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
    pub fn add_text_stl(input: &str) {
        info!("{:?}", input);
    }
}
