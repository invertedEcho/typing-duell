use std::time::Duration;

use ratatui::crossterm::{
    self,
    event::{Event, KeyCode},
};

pub fn handle_key_event() -> String {
    let is_key_event_available = crossterm::event::poll(Duration::from_millis(100)).unwrap();
    if is_key_event_available {
        let key_event = crossterm::event::read().unwrap();

        if let Event::Key(key) = key_event {
            match key.code {
                KeyCode::Char(char) => return handle_char(&char),
                _ => {}
            }
        };
    }

    return "ok".to_string();
}

pub const KEY_RETURN_QUIT: &str = "quit";
fn handle_char(char: &char) -> String {
    match char {
        'q' => KEY_RETURN_QUIT.to_string(),
        _ => "ok".to_string(),
    }
}
