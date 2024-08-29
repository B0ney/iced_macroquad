use crate::event_handler::{EventProxy, EventProxyHandler};
use crate::Renderer;

use iced_core::Size;
use iced_graphics::Viewport;
use macroquad::window::get_internal_gl;
use macroquad::{miniquad::window::screen_size, window::screen_dpi_scale};

pub(crate) struct Engine {
    pub renderer: Renderer,
    pub input_subscriber_id: usize,
}

impl Engine {
    fn new() -> Self {
        Self {
            input_subscriber_id: macroquad::input::utils::register_input_subscriber(),
            renderer: Renderer::new(unsafe { get_internal_gl().quad_context }),
        }
    }

    pub fn read_events<T: EventProxyHandler>(&self, proxy: &mut EventProxy<T>) {
        macroquad::input::utils::repeat_all_miniquad_input(proxy, self.input_subscriber_id);
    }

    pub fn fetch_viewport(&self) -> Viewport {
        let (width, height) = screen_size();
        Viewport::with_physical_size(
            Size::new(width as u32, height as u32),
            screen_dpi_scale() as f64,
        )
    }
}

pub(crate) mod global {
    use std::{cell::RefCell, sync::Once};

    use crate::engine::Engine;

    thread_local! {
        static ICED_CONTEXT: RefCell<Engine> = init_single_thread(|| RefCell::new(Engine::new()));
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

    pub fn iced_ctx_mut<T>(f: impl FnOnce(&mut Engine) -> T) -> T {
        ICED_CONTEXT.with_borrow_mut(f)
    }

    pub fn iced_ctx<T>(f: impl FnOnce(&Engine) -> T) -> T {
        ICED_CONTEXT.with_borrow(f)
    }
}
