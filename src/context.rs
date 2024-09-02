use crate::clipboard::Clipboard;
use crate::event_handler::{EventProxy, EventProxyWrapper};
use crate::renderer::engine::Engine;
use crate::renderer::Canvas;

use crate::mq::window::{dpi_scale, screen_size, set_mouse_cursor};
use crate::mq::CursorIcon;

use crate::macroquad::input::mouse_position;
use crate::macroquad::window::get_internal_gl;

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

    pub fn render(&mut self, canvas: &mut Canvas) {
        self.engine
            .present(unsafe { get_internal_gl().quad_context }, canvas)
    }

    pub fn dpi_scale(&self) -> f64 {
        dpi_scale() as f64
    }

    pub fn screen_size(&self) -> (u32, u32) {
        let (width, height) = screen_size();
        (width as u32, height as u32)
    }

    pub fn mouse_position(&self) -> (f32, f32) {
        mouse_position()
    }

    pub fn set_mouse_icon(&self, icon: CursorIcon) {
        set_mouse_cursor(icon)
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
