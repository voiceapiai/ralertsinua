use serde::Deserialize;

pub const DEFAULT_ALERTS_RESPONSE_STRING: &str = "NNNNNNNNNNNNNNNNNNNNNNNNNNN";

#[derive(
    Debug,
    Default,
    Deserialize,
    strum_macros::EnumProperty,
    strum_macros::AsRefStr,
    strum_macros::Display,
)]
pub enum AlertStatus {
    /// Active
    #[strum(props(icon = "🜸", color = "red"))]
    A,
    /// Partially active
    #[strum(props(icon = "🌤", color = "yellow"))]
    P,
    /// No information
    #[strum(props(icon = "🌣", color = "blue"))]
    #[default]
    N,
    /// Loading
    #[strum(props(icon = "↻", color = "white"))]
    L,
}

impl From<char> for AlertStatus {
    fn from(c: char) -> Self {
        match c {
            'A' => AlertStatus::A,
            'P' => AlertStatus::P,
            'L' => AlertStatus::L,
            _ => AlertStatus::N,
        }
    }
}
