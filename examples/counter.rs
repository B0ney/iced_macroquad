use iced_macroquad::{
    widget::{self, Theme},
    Iced,
};
use macroquad::prelude::*;

#[derive(Debug, Clone)]
enum Message {
    Add,
    Sub,
}

#[macroquad::main("macroqaud with iced")]
async fn main() {
    let mut ui: Iced<Message> = Iced::new();
    let mut messages: Vec<Message> = Vec::new();
    let mut counter = 0;

    loop {
        ui.interact_with(
            &mut messages,
            widget::row![
                widget::button("Add").on_press(Message::Add),
                widget::text(counter),
                widget::button("Sub").on_press(Message::Sub),
            ],
        );

        for message in messages.drain(..) {
            match message {
                Message::Add => counter += 1,
                Message::Sub => counter -= 1,
            }
        }

        ui.present();

        next_frame().await
    }
}
