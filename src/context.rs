use crate::clipboard::Clipboard;
use crate::event_handler::{EventProxy, EventProxyWrapper};
use crate::renderer::engine::Engine;

use macroquad::window::get_internal_gl;

pub(crate) struct Context {
    pub engine: Engine,
    pub clipboard: Clipboard,
    pub input_subscriber_id: usize,
}

impl Context {
    fn new() -> Self {
        Self {
            input_subscriber_id: macroquad::input::utils::register_input_subscriber(),
            engine: Engine::new(unsafe { get_internal_gl().quad_context }),
            clipboard: Clipboard::default(),
        }
    }

    pub fn read_events<T: EventProxy>(&self, event_proxy: T) {
        macroquad::input::utils::repeat_all_miniquad_input(
            &mut EventProxyWrapper(event_proxy),
            self.input_subscriber_id,
        );
    }
}

pub(crate) mod global {
    use std::{cell::RefCell, sync::Once};

    use crate::context::Context;

    thread_local! {
        static ICED_CONTEXT: RefCell<Context> = init_single_thread(|| RefCell::new(Context::new()));
    }

    fn init_single_thread<T>(init: impl FnOnce() -> T) -> T {
        try_init_single_thread(init).expect("Already initialized from another thread.")
    }

    fn try_init_single_thread<T>(init: impl FnOnce() -> T) -> Option<T> {
        static ONCE: Once = Once::new();
        let mut obj = None;
        ONCE.call_once(|| obj = Some(init()));
        obj
    }

    pub fn iced_ctx_mut<T>(f: impl FnOnce(&mut Context) -> T) -> T {
        ICED_CONTEXT.with_borrow_mut(f)
    }
}
