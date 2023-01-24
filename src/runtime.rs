use std::sync::Mutex;

use once_cell::sync::Lazy;
use wasm_bindgen::prelude::wasm_bindgen;
use winit::{
    event::Event,
    event_loop::{ControlFlow, EventLoopWindowTarget},
    window::Window,
};

use crate::{dom::Dom, gpu::WgpuContext, state::State};

#[wasm_bindgen]
pub struct Runtime {
    context: WgpuContext,
    window: Window,
    dom: Dom,
    state: &'static Lazy<Mutex<State>>,
}

impl Runtime {
    pub fn new(
        context: WgpuContext,
        window: Window,
        dom: Dom,
        state: &'static Lazy<Mutex<State>>,
    ) -> Self {
        Self {
            context,
            window,
            dom,
            state,
        }
    }

    pub fn main_loop<T>(
        &mut self,
        event: Event<()>,
        _target: &EventLoopWindowTarget<T>,
        control_flow: &mut ControlFlow,
    ) {
        // Log every event
        self.dom.event_logger.log_event(&event);

        match event {
            Event::NewEvents(_) => {}
            Event::WindowEvent { window_id, event } => {}
            Event::DeviceEvent { device_id, event } => {}
            Event::UserEvent(event) => {}
            Event::Suspended => {}
            Event::Resumed => {}
            Event::MainEventsCleared => {}
            Event::RedrawEventsCleared => {}
            Event::LoopDestroyed => {}
            Event::RedrawRequested(window_id)
                if window_id == self.window.id() =>
            {
                match self.context.render(&self.state.lock().unwrap()) {
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => {
                        self.context.resize(self.context.size)
                    }
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => {
                        self.dom.event_logger.log_message("Out of memory!");
                        *control_flow = ControlFlow::Exit
                    }
                    Err(e) => {
                        // Error!
                        self.dom.event_logger.log_message(&format!("{:?}", e));
                    }
                    _ => {}
                }
            }
            _ => (),
        }
    }
}
