use serde::Deserialize;

#[derive(
    Debug,
    Default,
    Deserialize,
    Clone,
    PartialEq,
    strum_macros::EnumProperty,
    strum_macros::AsRefStr,
    strum_macros::Display,
)]
pub enum AlertStatus {
    /// Active
    #[strum(props(icon = "🜸", color = "red"))] // 🔴
    A,
    /// Partially active
    #[strum(props(icon = "🌤", color = "yellow"))] // 🟡
    P,
    /// No information
    #[strum(props(icon = "🌣", color = "blue"))] // 🟢
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
