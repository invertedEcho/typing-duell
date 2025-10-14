use std::time::Duration;

use ratatui::crossterm::{
    self,
    event::{Event, KeyCode},
};

use crate::{AppState, CurrentScreen};

pub const KEY_RETURN_QUIT: &str = "quit";

pub fn handle_key_event(app_state: &mut AppState) -> Result<&'static str, &'static str> {
    let is_key_event_available = crossterm::event::poll(Duration::from_millis(100)).unwrap();
    if is_key_event_available {
        let key_event = crossterm::event::read().unwrap();

        return if let Event::Key(key) = key_event {
            match key.code {
                KeyCode::Char(char) => return handle_char(&char, app_state),
                KeyCode::Esc => handle_escape(app_state),
                KeyCode::Backspace => {
                    handle_backspace(app_state);
                    return Ok("ok");
                }
                _ => {
                    return Ok("ok");
                }
            }
        } else {
            return Ok("ok");
        };
    }

    return Ok("ok");
}

fn handle_char(char: &char, app_state: &mut AppState) -> Result<&'static str, &'static str> {
    match char {
        // TODO: i want some kind of event system for stuff like this, maybe use mpsc like in
        // file-explorer-tui again?
        'q' => handle_lower_case_q_char(app_state),
        'p' => handle_lowercase_p_char(app_state),
        char => handle_all_char(*char, app_state),
    }
}

fn handle_lowercase_p_char(app_state: &mut AppState) -> Result<&'static str, &'static str> {
    match app_state.current_screen {
        CurrentScreen::MainMenu => {
            app_state.current_screen = CurrentScreen::Playing;
        }
        CurrentScreen::Playing => {
            app_state.current_text_to_type_index += 1;
            app_state.user_text.push('p');
        }
    }
    Ok("ok")
}

fn handle_lower_case_q_char(app_state: &mut AppState) -> Result<&'static str, &'static str> {
    match app_state.current_screen {
        CurrentScreen::MainMenu => Ok(KEY_RETURN_QUIT),
        CurrentScreen::Playing => {
            app_state.current_text_to_type_index += 1;
            app_state.user_text.push('q');
            Ok("ok")
        }
    }
}

fn handle_all_char(char: char, app_state: &mut AppState) -> Result<&'static str, &'static str> {
    app_state.current_text_to_type_index += 1;
    app_state.user_text.push(char);
    Ok("ok")
}

fn handle_escape(app_state: &mut AppState) -> Result<&'static str, &'static str> {
    match app_state.current_screen {
        CurrentScreen::MainMenu => Ok(KEY_RETURN_QUIT),
        CurrentScreen::Playing => {
            // TODO: implement paused
            app_state.current_screen = CurrentScreen::MainMenu;
            Ok("ok")
        }
    }
}

fn handle_backspace(app_state: &mut AppState) {
    if app_state.current_text_to_type_index != 0 {
        app_state.current_text_to_type_index -= 1;
    }
    if app_state.user_text.chars().count() != 0 {
        app_state.user_text.pop();
    }
}
