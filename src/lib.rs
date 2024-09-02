mod context;
mod convert;
mod event_handler;
mod interface;
mod renderer;
mod clipboard;

use macroquad;
use macroquad::miniquad as mq;

// pub use iced_core::*;
pub use iced_widget as widget;

pub type Renderer = renderer::Canvas;

pub use interface::Interface;
