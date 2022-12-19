//! Basic DOM communication boilerplate and event logging.

use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlCanvasElement};
use winit::event::Event;

pub struct EventLogger {
    inner: Element,
}

impl Default for EventLogger {
    fn default() -> Self {
        Self {
            inner: web_sys::window()
                .and_then(|w| w.document())
                .and_then(|d| d.get_element_by_id("event_logger"))
                .expect("can't find event logger element"),
        }
    }
}

impl EventLogger {
    pub fn log_event(&self, event: &Event<()>) {
        if let Event::WindowEvent { event, .. } = &event {
            let log = web_sys::window()
                .and_then(|w| w.document())
                .and_then(|d| d.create_element("li").ok())
                .expect("could not create log");
            log.set_text_content(Some(&format!("{:?}", event)));

            self.inner
                .insert_before(&log, self.inner.first_child().as_ref())
                .expect("could not append log");
        }
    }

    pub fn log_message(&self, message: &str) {
        let log = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.create_element("li").ok())
            .expect("could not create message");
        log.set_text_content(Some(message));

        self.inner
            .insert_before(&log, self.inner.first_child().as_ref())
            .expect("could not append message");
    }
}

pub struct Dom {
    pub canvas: HtmlCanvasElement,
    pub event_logger: EventLogger,
}

impl Default for Dom {
    fn default() -> Self {
        Self {
            canvas: web_sys::window()
                .and_then(|w| w.document())
                .and_then(|d| d.get_element_by_id("wgpu_canvas"))
                .map(|e| e.unchecked_into::<HtmlCanvasElement>())
                .expect("can't find canvas element"),
            event_logger: EventLogger::default(),
        }
    }
}
