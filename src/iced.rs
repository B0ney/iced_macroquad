// https://github.com/iced-rs/iced/blob/master/src/lib.rs for reference

//! Re-exports nearly all of Iced's API

pub use iced_core::alignment;
pub use iced_core::border;
pub use iced_core::event;
pub use iced_core::color;
pub use iced_core::gradient;
pub use iced_core::padding;
pub use iced_core::theme;
pub use iced_core::{
    Alignment, Background, Border, Color, ContentFit, Degrees, Gradient, Length, Padding, Pixels,
    Point, Radians, Rectangle, Rotation, Shadow, Size, Theme, Transformation, Vector,
};

pub use alignment::Horizontal::{Left, Right};
pub use alignment::Vertical::{Bottom, Top};
pub use Alignment::Center;
pub use Length::{Fill, FillPortion, Shrink};

pub mod font {
    //! Load and use fonts.
    pub use iced_core::font::*;

    pub fn load(fonts: Vec<std::borrow::Cow<'static, [u8]>>) {
        let mut font_system = iced_graphics::text::font_system()
            .write()
            .expect("Acquire global font system");

        for font in fonts {
            font_system.load_font(font)
        }
    }
}

#[allow(hidden_glob_reexports)]
pub mod widget {
    //! Use the built-in widgets or create your own.
    pub use iced_widget::*;

    // We hide the re-exported modules by `iced_widget`
    mod core {}
    mod graphics {}
    mod native {}
    mod renderer {}
    mod style {}
    mod runtime {}
}

pub mod overlay {
    //! Display interactive elements on top of other widgets.

    /// A generic overlay.
    ///
    /// This is an alias of an [`overlay::Element`] with a default `Renderer`.
    ///
    /// [`overlay::Element`]: crate::core::overlay::Element
    pub type Element<
        'a,
        Message,
        Theme = super::Theme,
        Renderer = super::Renderer,
    > = iced_core::overlay::Element<'a, Message, Theme, Renderer>;

    pub use iced_widget::overlay::*;
}

pub mod advanced {
    pub use iced_core::widget::*;
    
    pub use iced_core::clipboard::{self, Clipboard};
    pub use iced_core::image;
    pub use iced_core::layout::{self, Layout};
    pub use iced_core::mouse;
    pub use iced_core::overlay::{self, Overlay};
    pub use iced_core::renderer::{self, Renderer};
    pub use iced_core::svg;
    pub use iced_core::text::{self, Text};
    pub use iced_core::Shell;
    pub use iced_graphics;
}

pub type Renderer = iced_tiny_skia::Renderer<iced_tiny_skia::color_profile::RGBA>;
pub type Element<'a, Message, Theme> = iced_core::Element<'a, Message, Theme, Renderer>;
