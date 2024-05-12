use ralertsinua_models::AlertStatus;
use ratatui::prelude::*;

pub trait WithLineItems {
    /// Builds new [`Line`] with styled text
    fn get_styled_line_by_status<'a>(
        text: &str,
        status: &AlertStatus,
        is_selected: bool,
    ) -> Line<'a> {
        use std::str::FromStr;
        use strum::EnumProperty;

        let text: Text = Text::from(text);

        let icon: &str = status.get_str("icon").unwrap();
        let color_str: &str = status.get_str("color").unwrap();
        let color: Color = Color::from_str(color_str).unwrap();
        let mut line: Line = Line::from(format!("{} {}", icon, text)).style(color);

        if is_selected {
            line = line.add_modifier(Modifier::BOLD);
        }

        line = match status {
            AlertStatus::A => line
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::RAPID_BLINK),
            AlertStatus::P => line.add_modifier(Modifier::ITALIC),
            AlertStatus::L => line.add_modifier(Modifier::DIM),
            _ => line,
        };

        line
    }

    /// Builds new [`Line`] with styled text
    fn get_styled_line_icon_by_status<'a>(
        status: &AlertStatus,
        is_selected: bool,
    ) -> Line<'a> {
        use std::str::FromStr;
        use strum::EnumProperty;

        let icon: &str = status.get_str("icon").unwrap();
        let color_str: &str = status.get_str("color").unwrap();
        let color: Color = Color::from_str(color_str).unwrap();
        let mut line: Line = Line::from(icon).style(color);

        if is_selected {
            line = line.add_modifier(Modifier::BOLD);
        }

        line = match status {
            AlertStatus::A => line
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::RAPID_BLINK),
            AlertStatus::P => line.add_modifier(Modifier::ITALIC),
            AlertStatus::L => line.add_modifier(Modifier::DIM),
            _ => line,
        };

        line
    }
}
