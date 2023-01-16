use std::sync::Mutex;

use once_cell::sync::Lazy;
use wasm_bindgen::prelude::wasm_bindgen;
use winit::{
    event::Event,
    event_loop::{ControlFlow, EventLoopWindowTarget},
    window::Window,
};

use crate::{dom::Dom, render::WgpuContext, state::State};

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
                //info!("Event::WindowEvent: {event:?}");
            }
            Event::DeviceEvent { device_id, event } => {
                //info!("Event::DeviceEvent: {event:?}");
            }
            Event::UserEvent(event) => {
                //info!("Event::DeviceEvent: {event:?}");
            }
            Event::Suspended => {}
            Event::Resumed => {}
            Event::MainEventsCleared => {}
            Event::RedrawRequested(_) => {}
            Event::RedrawEventsCleared => {}
            Event::LoopDestroyed => {}
        }
    }
}
