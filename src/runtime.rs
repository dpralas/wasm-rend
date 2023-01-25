use std::sync::Mutex;

use log::info;
use once_cell::sync::Lazy;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::info;
use winit::{
    event::{DeviceEvent, Event, WindowEvent},
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
            Event::WindowEvent { window_id, event } => {
                if event == WindowEvent::Focused(true) {
                    self.context.render(&self.state.lock().unwrap()).ok();
                }
            }
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
                self.context.render(&self.state.lock().unwrap()).ok();
            }
            _ => (),
        }
    }
}
