use iced_macroquad::widget::{button, row, text};
use iced_macroquad::Interface;

use macroquad::prelude::*;

#[derive(Debug, Clone)]
enum Message {
    Add,
    Sub,
}

#[macroquad::main("macroqaud with iced")]
async fn main() {
    let mut ui = Interface::<Message>::new();
    let mut messages = Vec::new();
    let mut counter = 0;

    loop {
        for message in messages.drain(..) {
            match message {
                Message::Add => {
                    println!("Increment");
                    counter += 1
                }
                Message::Sub => {
                    println!("Decrement");
                    counter -= 1
                }
            }
        }

        ui.view(
            &mut messages,
            row![
                button("Add +").width(32).height(16).on_press(Message::Add),
                text(counter).width(32),
                button("Sub -").width(32).height(16).on_press(Message::Sub),
            ],
        );

        next_frame().await
    }
}
