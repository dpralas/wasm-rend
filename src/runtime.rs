use log::info;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopWindowTarget},
    window::Window,
};

use crate::{dom::Dom, render::WgpuContext};

pub struct Runtime {
    context: WgpuContext,
    window: Window,
    dom: Dom,
}

impl Runtime {
    pub fn new(context: WgpuContext, window: Window, dom: Dom) -> Self {
        Self {
            context,
            window,
            dom,
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
