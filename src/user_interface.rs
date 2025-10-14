use color_eyre::owo_colors::OwoColorize;
use log::info;
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::{AppState, CurrentScreen};

pub fn draw_widgets_to_frame(frame: &mut Frame, app_state: &mut AppState) {
    match app_state.current_screen {
        CurrentScreen::MainMenu => draw_main_menu_screen_to_frame(frame),
        CurrentScreen::Playing => draw_playing_screen_to_frame(frame, app_state),
    }
}

fn draw_main_menu_screen_to_frame(frame: &mut Frame) {
    let root_layout = Layout::vertical([
        Constraint::Percentage(20),
        Constraint::Percentage(80),
        Constraint::Percentage(20),
    ]);

    let [_, center_layout, _] = root_layout.areas(frame.area());

    let center_block = Block::new().title("Main Menu").borders(Borders::all());
    let title = Paragraph::new("Press P to start racing!").block(center_block);

    frame.render_widget(title, center_layout);
}

fn draw_playing_screen_to_frame(frame: &mut Frame, app_state: &mut AppState) {
    let root_layout = Layout::vertical([
        Constraint::Percentage(15),
        Constraint::Percentage(70),
        Constraint::Percentage(15),
    ]);

    let [_, center_layout, _] = root_layout.areas(frame.area());

    let title_right = format!("WPM: {}", app_state.wpm);
    let center_block = Block::new()
        .title_top(Line::from("Now Playing").left_aligned())
        .title_top(Line::from(title_right).right_aligned())
        .borders(Borders::all());

    let mut text_as_spans = vec![];
    for (index, char) in app_state.text_to_type.chars().enumerate() {
        if index == app_state.current_text_to_type_index {
            text_as_spans.push(Span::styled(
                char.to_string(),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::UNDERLINED),
            ));
        } else {
            text_as_spans.push(Span::styled(
                char.to_string(),
                Style::default().fg(Color::White),
            ));
        }
    }

    for (index, user_char) in app_state.user_text.chars().enumerate() {
        let current_char_from_text_to_display = &mut text_as_spans[index];

        if current_char_from_text_to_display.content == user_char.to_string() {
            current_char_from_text_to_display.style = Style::default().fg(Color::Green);
        } else {
            current_char_from_text_to_display.style = Style::default().fg(Color::Red);
        }
    }

    let text_as_paragraph = Paragraph::new(Line::from(text_as_spans))
        .block(center_block)
        .wrap(Wrap { trim: true });

    frame.render_widget(text_as_paragraph, center_layout);
}
