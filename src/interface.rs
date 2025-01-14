use std::marker::PhantomData;

use iced_core::event::Status;
use iced_core::mouse::{Cursor, Interaction};
use iced_core::renderer::Style;
use iced_core::widget::{operation, Operation};
use iced_core::{Element, Point};

use iced_runtime::{user_interface::Cache, UserInterface};

use crate::cursor::CursorSubscriber;
use crate::iced::Renderer;

use crate::context::{global, Context, MqCursor};
use crate::convert;

pub struct Interface<Message, Theme = iced_core::Theme> {
    in_events: Vec<iced_core::Event>,
    operations: Vec<Box<dyn Operation>>,
    ui_cache: Option<Cache>,
    theme: Theme,
    interacted: bool,
    captured: bool,
    _message: PhantomData<Message>,
}

impl<Message, Theme: Default> Interface<Message, Theme> {
    pub fn new() -> Self {
        Self::new_themed(Theme::default())
    }
}

impl<Message, Theme> Interface<Message, Theme> {
    pub fn new_themed(theme: Theme) -> Self {
        Self {
            in_events: Vec::new(),
            ui_cache: None,
            theme,
            interacted: false,
            operations: Vec::new(),
            captured: false,
            _message: PhantomData,
        }
    }

    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme
    }

    /// Returns true if the interface was interacted with since the last view
    pub fn interacted(&self) -> bool {
        self.interacted
    }

    /// Returns true if the interface has captured the cursor since the last view
    pub fn captured(&self) -> bool {
        self.captured
    }

    /// Perform a widget operation
    pub fn operate(&mut self, operation: impl Operation + 'static) {
        self.operations.push(Box::new(operation));
    }

    /// Interact with, and view the UI. All interactions will be pushed to messages.
    pub fn view<'a>(
        &mut self,
        messages: &mut Vec<Message>,
        ui: Element<'a, Message, Theme, Renderer>,
    ) {
        global::iced_ctx_mut(|ctx| self.present(ctx, messages, &mut MqCursor, ui.into()));
    }

    /// Interact with, and view the UI. All interactions will be pushed to messages.
    pub fn view_advanced<'a>(
        &mut self,
        messages: &mut Vec<Message>,
        cursor: &mut dyn CursorSubscriber,
        ui: Element<'a, Message, Theme, Renderer>,
    ) {
        global::iced_ctx_mut(|ctx| self.present(ctx, messages, cursor, ui.into()));
    }

    fn present(
        &mut self,
        ctx: &mut Context,
        messages: &mut Vec<Message>,
        cursor_icon: &mut dyn CursorSubscriber,
        ui: Element<'_, Message, Theme, Renderer>,
    ) {
        let viewport = ctx.viewport();
        // Build the interface.
        let mut interface = UserInterface::build(
            ui,
            viewport.logical_size(),
            self.ui_cache.take().unwrap_or_default(),
            &mut ctx.renderer,
        );

        // Perform widget operations, if any.
        for operation in self.operations.drain(..) {
            let mut current_operation = Some(operation);

            while let Some(mut operation) = current_operation.take() {
                interface.operate(&mut ctx.renderer, &mut operation);

                if let operation::Outcome::Chain(next_operation) = operation.finish() {
                    current_operation = Some(next_operation)
                }
            }
        }

        // Fetch all external inputs.
        self.in_events.clear();
        ctx.read_events(&mut self.in_events);

        // Update the interface by processing the events.
        let cursor = Cursor::Available(Point::from(ctx.mouse_position()));
        let (_, statuses) = interface.update(
            &self.in_events,
            cursor,
            &mut ctx.renderer,
            &mut ctx.clipboard,
            messages,
        );

        // Draw the interface onto the canvas.
        let interaction = interface.draw(&mut ctx.renderer, &self.theme, &Style::default(), cursor);

        self.captured = statuses
            .into_iter()
            .any(|status| status == Status::Captured);

        // Update mouse cursor.
        if interaction == Interaction::None {
            if self.interacted {
                self.interacted = false;
                cursor_icon.reset();
            }
        } else {
            cursor_icon.update(convert::cursor_icon(interaction));
            self.interacted = true;
        }

        // Cache the interface for reuse the next time view is called.
        self.ui_cache = Some(interface.into_cache());

        // Render what's drawn on the canvas to the screen.
        ctx.present(&viewport);
    }
}
