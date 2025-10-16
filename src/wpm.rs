use std::time::{SystemTime, UNIX_EPOCH};

use log::warn;

use crate::AppState;

pub fn calculate_words_per_minute(app_state: &AppState) -> usize {
    let Some(type_test_timestamp) = app_state.type_test_timestamp else {
        warn!(
            "calculate_character_per_minute was called without app_state.type_test_timestamp set!"
        );
        return 0;
    };

    let seconds_elapsed = get_unix_timestamp() - type_test_timestamp;

    let characters_typed_count = app_state.user_text.chars().count();

    let words_typed = characters_typed_count / 5;

    if seconds_elapsed < 60 {
        let factor = 60 / seconds_elapsed;
        return words_typed * (factor as usize);
    } else {
        let minutes_elapsed = seconds_elapsed / 60;
        words_typed / minutes_elapsed as usize
    }
}

pub fn get_unix_timestamp() -> u64 {
    return SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
}
