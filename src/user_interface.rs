use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    widgets::{Block, Borders, Paragraph},
};

use crate::{AppState, AppStateEnum};

pub fn draw_widgets_to_frame(frame: &mut Frame, app_state: &mut AppState) {
    match app_state.app_state {
        AppStateEnum::MainMenu => draw_main_menu_screen_to_frame(frame),
        _ => {}
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

fn draw_playing_screen_to_frame() {}
