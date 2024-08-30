use std::any::TypeId;

use crate::event_handler::{EventProxy, EventProxyWrapper};
use crate::renderer::engine::Engine;
use crate::renderer::Canvas;
use crate::Renderer;

use iced_core::Size;
use iced_graphics::Viewport;
use macroquad::window::get_internal_gl;
use macroquad::{miniquad::window::screen_size, window::screen_dpi_scale};
use rustc_hash::FxHashMap;

pub(crate) struct Context {
    pub engine: Engine,
    pub canvases: FxHashMap<TypeId, Canvas>,
    pub input_subscriber_id: usize,
}

impl Context {
    fn new() -> Self {
        Self {
            input_subscriber_id: macroquad::input::utils::register_input_subscriber(),
            engine: Engine::new(unsafe { get_internal_gl().quad_context }),
            canvases: FxHashMap::default(),
        }
    }

    pub fn fetch_canvas<Message>(&mut self) -> &mut Canvas {
        self.canvases
            .entry(typeid::of::<Message>())
            .or_insert_with(|| Canvas::new())
    }

    pub fn read_events<T: EventProxy>(&self, event_proxy: T) {
        macroquad::input::utils::repeat_all_miniquad_input(
            &mut EventProxyWrapper(event_proxy),
            self.input_subscriber_id,
        );
    }

    pub fn fetch_viewport(&self) -> Viewport {
        let (width, height) = screen_size();
        Viewport::with_physical_size(
            Size::new(width as u32, height as u32),
            screen_dpi_scale() as f64,
        )
    }

    pub fn present<Message>(&mut self) {
        if let Some(canvas) = self.canvases.get_mut(&typeid::of::<Message>()) {
            canvas.present(&mut self.engine)
        }
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

    pub fn iced_ctx<T>(f: impl FnOnce(&Context) -> T) -> T {
        ICED_CONTEXT.with_borrow(f)
    }
}
