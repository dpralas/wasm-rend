mod dom;
mod gpu;
mod io;
mod model;
mod runtime;
mod state;

use std::sync::Mutex;

use dom::Dom;
use gpu::WgpuContext;
use log::info;
use once_cell::sync::Lazy;
use wasm_bindgen::prelude::*;
use winit::{
    dpi::LogicalSize, event_loop::EventLoop,
    platform::web::WindowBuilderExtWebSys, window::WindowBuilder,
};

use crate::{runtime::Runtime, state::State};

static STATE: Lazy<Mutex<State>> = Lazy::new(|| Mutex::new(State::default()));

#[wasm_bindgen]
pub fn add_file(file: &[u8]) {
    STATE.lock().unwrap().add_stl(file);
}

#[wasm_bindgen(start)]
pub async fn run() {
    console_log::init_with_level(log::Level::Debug)
        .expect("could not initialize logger");
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let dom = Dom::default();
    let canvas = &dom.canvas;
    let (width, height) = (canvas.client_width(), canvas.client_height());

    let mut context = WgpuContext::new(canvas).await;

    // Create window
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_canvas(Some(canvas.to_owned()))
        .build(&event_loop)
        .map(|w| {
            // Set initial view port -- ** This isn't what we want! **
            // We want the canvas to always fit to the document.
            w.set_inner_size(LogicalSize::new(width, height));
            w
        })
        .expect("Could not build window");
    info!("Created window");

    context.add_shader(
        "solid.vert",
        include_str!("../assets/shaders/solid.vert.wgsl"),
    );
    context.add_shader(
        "solid.frag",
        include_str!("../assets/shaders/solid.frag.wgsl"),
    );

    let mut runtime = Runtime::new(context, window, dom, &STATE);
    event_loop.run(move |event, target, control_flow| {
        runtime.main_loop(event, target, control_flow)
    });
}
