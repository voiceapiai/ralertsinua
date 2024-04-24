#![allow(unused)]
use lazy_static::lazy_static;
use ratatui::style::{palette::material, Color};

lazy_static! {
    // Colors
    pub static ref DEFAULT_COLOR: Color = Color::Reset;
    pub static ref TEXT_COLOR: Color = Color::LightBlue;
    pub static ref MARKER_COLOR: Color = Color::LightYellow;
    pub static ref NORMAL_ROW_COLOR: Color = Color::Reset;
    pub static ref ALERT_ROW_COLOR: Color = Color::Red;
    pub static ref SELECTED_STYLE_FG: Color = Color::Blue;
    pub static ref MATERIAL_TEXT_COLOR: Color = material::YELLOW.c900;

    /// @see https://serde.rs/custom-date-format.html
    pub static ref FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";
}
