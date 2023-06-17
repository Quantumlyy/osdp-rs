use crate::constants::{ASCII_DISPLAY_RANGE_START, ASCII_DISPLAY_RANGE_END};

pub fn string_non_ascii(text: &str) -> String {
    text.replace(|c: char| !c.is_ascii(), "")
}

pub fn filter_ascii_display_characters(text: &str) -> String {
    text.chars()
        .filter(|c| {
            let c = *c as u8;
            c >= ASCII_DISPLAY_RANGE_START && c <= ASCII_DISPLAY_RANGE_END
        })
        .collect()
}
