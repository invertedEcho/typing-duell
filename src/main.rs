use color_eyre::Result;
use log::warn;
use ratatui::DefaultTerminal;

use crate::{
    logger::setup_logger_handle,
    texts::LOREM_IPSUM_TEXT,
    user_input::{KEY_RETURN_QUIT, handle_key_event},
    user_interface::draw_widgets_to_frame,
};

mod game;
mod logger;
mod texts;
mod user_input;
mod user_interface;

#[derive(Default)]
struct AppState {
    current_screen: CurrentScreen,
    // TODO: put somewhere else
    wpm: usize,
    // TODO: this should only be set when we are in playing state
    text_to_type: String,
    current_text_to_type_index: usize,
    /// The text that was written by the user
    user_text: String,
}

#[derive(Default)]
enum CurrentScreen {
    #[default]
    MainMenu,
    Playing,
}

fn main() -> Result<()> {
    setup_logger_handle();

    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();

    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let mut app_state = AppState {
        text_to_type: LOREM_IPSUM_TEXT.to_string(),
        ..Default::default()
    };

    loop {
        match handle_key_event(&mut app_state) {
            Ok(res) => {
                if res == KEY_RETURN_QUIT {
                    break Ok(());
                }
            }
            Err(error) => {
                warn!("Received error: {}", error);
            }
        }

        terminal.draw(|frame| draw_widgets_to_frame(frame, &mut app_state))?;
    }
}
