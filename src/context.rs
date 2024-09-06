use iced_core::{Font, Pixels, Size};
use iced_graphics::Viewport;
use iced_tiny_skia;

use crate::clipboard::Clipboard;
use crate::event_handler::{EventProxy, EventProxyWrapper};

use crate::mq::window::{dpi_scale, screen_size, set_mouse_cursor};
use crate::mq::CursorIcon;

use crate::macroquad::input::mouse_position;

pub(crate) struct Context {
    pub renderer: iced_tiny_skia::Renderer,
    pub compositor: renderer::Compositor,
    pub clipboard: Clipboard,
    pub input_subscriber_id: usize,
}

impl Context {
    fn new() -> Self {
        let (width, height) = screen_size();

        Self {
            input_subscriber_id: macroquad::input::utils::register_input_subscriber(),
            renderer: iced_tiny_skia::Renderer::new(Font::DEFAULT, Pixels(12.0)),
            compositor: renderer::Compositor::new(Size::new(width as u32, height as u32)),
            clipboard: Clipboard::default(),
        }
    }

    pub fn read_events<T: EventProxy>(&self, event_proxy: T) {
        macroquad::input::utils::repeat_all_miniquad_input(
            &mut EventProxyWrapper(event_proxy),
            self.input_subscriber_id,
        );
    }

    pub fn present(&mut self, viewport: &Viewport) {
        self.compositor.present(&mut self.renderer, &viewport);
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

    pub fn viewport(&self) -> Viewport {
        Viewport::with_physical_size(self.screen_size().into(), self.dpi_scale())
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

mod renderer {
    use iced_core::{Color, Rectangle, Size};
    use iced_graphics::{damage, Viewport};
    use iced_tiny_skia::{Layer, Renderer};
    use macroquad::texture::Texture2D;

    pub fn create_texture(size: Size<u32>, buffer: &[u32]) -> Texture2D {
        let texture = Texture2D::from_rgba8(
            size.width as u16,
            size.height as u16,
            bytemuck::cast_slice(buffer),
        );
        texture.set_filter(macroquad::texture::FilterMode::Nearest);
        texture
    }

    pub struct Surface {
        dirty: bool,
        buffer: Vec<u32>,
        texture: Texture2D,
        size: Size<u32>,
    }

    impl Surface {
        pub fn new(size: Size<u32>) -> Self {
            let buffer = vec![0; (size.width * size.height) as usize];
            Self {
                dirty: true,
                size,
                texture: create_texture(size, &buffer),
                buffer,
            }
        }

        pub fn data_mut(&mut self) -> &mut [u8] {
            self.dirty = true;
            bytemuck::cast_slice_mut(&mut self.buffer)
        }

        pub fn data(&self) -> &[u8] {
            bytemuck::cast_slice(&self.buffer)
        }

        pub fn resize(&mut self, size: Size<u32>) {
            self.size = size;
            self.buffer
                .resize(size.width as usize * size.height as usize, 0);
            self.texture = create_texture(size, &self.buffer);
            self.dirty = true;
        }

        pub fn update_texture(&mut self) {
            if self.dirty {
                self.texture
                    .update_from_bytes(self.size.width, self.size.height, self.data());
                self.dirty = false
            }
        }
    }

    pub struct Compositor {
        mask: tiny_skia::Mask,
        old_frame: Option<Vec<Layer>>,
        surface: Surface,
    }

    impl Compositor {
        pub fn new(size: Size<u32>) -> Self {
            Self {
                mask: tiny_skia::Mask::new(size.width, size.height).unwrap(),
                old_frame: None,
                surface: Surface::new(size),
            }
        }

        pub fn maybe_resize_buffers(&mut self, new_size: Size<u32>) {
            if self.surface.size != new_size {
                self.resize(new_size)
            }
        }

        pub fn resize(&mut self, new_size: Size<u32>) {
            self.surface.resize(new_size);
            self.mask = tiny_skia::Mask::new(new_size.width, new_size.height).expect("create mask");
            self.old_frame = None;
        }

        pub fn render_to_surface(&mut self, renderer: &mut Renderer, viewport: &Viewport) {
            let redraw = || vec![Rectangle::with_size(viewport.logical_size())];

            let damage = self
                .old_frame
                .as_ref()
                .map(|last_layers| {
                    damage::diff(
                        last_layers,
                        renderer.layers(),
                        |layer| vec![layer.bounds],
                        Layer::damage,
                    )
                })
                .unwrap_or_else(redraw);

            if damage.is_empty() {
                return;
            }

            self.old_frame = Some(renderer.layers().to_vec());

            let damage = damage::group(damage, Rectangle::with_size(viewport.logical_size()));
            let physical_size = viewport.physical_size();

            let mut pixels = tiny_skia::PixmapMut::from_bytes(
                self.surface.data_mut(),
                physical_size.width,
                physical_size.height,
            )
            .expect("Create pixel map");

            renderer.draw(
                &mut pixels,
                &mut self.mask,
                &viewport,
                &damage,
                Color::TRANSPARENT,
                &[] as &[&str],
            );
        }

        pub fn draw_texture(&mut self) {
            self.surface.update_texture();

            macroquad::texture::draw_texture(
                &self.surface.texture,
                0.0,
                0.0,
                macroquad::color::WHITE,
            );
        }

        pub fn present(&mut self, renderer: &mut Renderer, viewport: &Viewport) {
            self.maybe_resize_buffers(viewport.physical_size());
            self.render_to_surface(renderer, viewport);
            self.draw_texture();
        }
    }
}
