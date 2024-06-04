use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Default,
    Deserialize,
    Serialize,
    Clone,
    PartialEq,
    strum_macros::EnumString,
    strum_macros::EnumProperty,
    strum_macros::AsRefStr,
    strum_macros::Display,
)]
// coveralls-ignore-next-line
pub enum AlertStatus {
    /// Active
    #[strum(to_string = "Active", props(icon = "🜸", color = "red"))] // 🔴
    A,
    /// Partially active
    #[strum(to_string = "Partial", props(icon = "🌤", color = "yellow"))] // 🟡
    P,
    /// No information
    #[strum(to_string = "No info", props(icon = "🌣", color = "blue"))] // 🟢
    #[default]
    N,
    /// Loading
    #[strum(to_string = "Loading", props(icon = "↻", color = "gray"))]
    L,
    /// Offline
    #[strum(to_string = "Offline", props(icon = "?", color = "darkgray"))]
    O,
}

// coveralls-ignore-next-line
impl From<char> for AlertStatus {
    fn from(c: char) -> Self {
        match c {
            'A' => AlertStatus::A,
            'P' => AlertStatus::P,
            'L' => AlertStatus::L,
            'O' => AlertStatus::O,
            _ => AlertStatus::N,
        }
    }
}
