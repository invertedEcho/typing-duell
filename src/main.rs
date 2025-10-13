use color_eyre::Result;
use ratatui::DefaultTerminal;

use crate::{
    user_input::{KEY_RETURN_QUIT, handle_key_event},
    user_interface::draw_widgets_to_frame,
};

mod texts;
mod user_input;
mod user_interface;

#[derive(Default)]
struct AppState {
    app_state: AppStateEnum,
}

#[derive(Default)]
enum AppStateEnum {
    #[default]
    MainMenu,
    Playing,
}

fn main() -> Result<()> {
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    return result;
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let mut app_state = AppState::default();

    loop {
        let result = handle_key_event();
        if result == KEY_RETURN_QUIT {
            break Ok(());
        }

        terminal.draw(|frame| draw_widgets_to_frame(frame, &mut app_state))?;
    }
}
