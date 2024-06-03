use michie::memoized;
use ralertsinua_models::AlertStatus;
use ratatui::{
    layout::{Constraint::*, Offset},
    prelude::*,
};
use std::collections::HashMap;
use std::{rc::Rc, str::FromStr};
use strum::EnumProperty;
#[allow(unused_imports)]
use tracing::{debug, info};

use crate::layout::*;

pub fn get_color_by_status(status: &AlertStatus) -> Color {
    let color_str: &str = status.get_str("color").unwrap();
    let color: Color = Color::from_str(color_str).unwrap();
    color
}

/// Builds new [`Line`] with styled text
// #[memoized(key_expr = input, store_type = HashMap<usize, usize>)]
pub fn get_styled_line_by_status<'a, S>(
    text: S,
    status: &AlertStatus,
    is_selected: &bool,
) -> Line<'a>
where
    S: Into<String>,
{
    let icon: &str = status.get_str("icon").unwrap();
    let color = get_color_by_status(status);
    let mut line: Line = Line::from(format!("{} {}", icon, text.into())).style(color);

    if *is_selected {
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
pub fn get_styled_line_icon_by_status<'a>(
    status: &AlertStatus,
    is_selected: &bool,
) -> Line<'a> {
    let icon: &str = status.get_str("icon").unwrap();
    let color = get_color_by_status(status);
    let mut line: Line = Line::from(icon).style(color);

    if *is_selected {
        line = line.add_modifier(Modifier::BOLD);
    }

    line = match status {
        AlertStatus::A => line
            .add_modifier(Modifier::BOLD)
            .add_modifier(Modifier::RAPID_BLINK),
        AlertStatus::P => line.add_modifier(Modifier::ITALIC),
        AlertStatus::L | AlertStatus::O => {
            line.add_modifier(Modifier::DIM).style(Color::DarkGray)
        }
        _ => line,
    };

    line
}

pub fn get_title_with_online_status<'a, S>(text: S, is_online: &bool) -> Line<'a>
where
    S: Into<String>,
{
    #[rustfmt::skip]
        let suffix: &str = match is_online { true => "", false => ": Offline", };
    let mut line: Line = Line::from(text.into() + suffix);

    if !is_online {
        line = line.add_modifier(Modifier::DIM).style(Color::DarkGray)
    }

    line
}

pub fn centered_rect(r: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Percentage((100 - percent_y) / 2),
            Percentage(percent_y),
            Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Percentage((100 - percent_x) / 2),
            Percentage(percent_x),
            Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

#[memoized(key_expr = (r, p_x, p_y), store_type = HashMap<(Rect, u16, u16), Rect>)]
pub fn get_bottom_left_rect(r: Rect, p_x: u16, p_y: u16) -> Rect {
    let vertical: Rc<[Rect]> = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Percentage(100 - p_y), Percentage(p_y)])
        .split(r);
    let horizontal: Rc<[Rect]> = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Percentage(p_x), Percentage(100 - p_x)])
        .split(vertical[1]);
    let area = horizontal[0].offset(Offset { x: 1, y: -1 });
    debug!(target:"app", "calculated area for bottom left rect: {:?}", horizontal[0]);

    area
}

#[memoized(key_expr = (frame_size, cmp_name, cmp_area), store_type = HashMap<(Rect, &'static str, LayoutArea), Rect>)]
pub fn get_component_area(
    frame_size: Rect,
    cmp_name: &'static str,
    cmp_area: LayoutArea,
) -> Rect {
    let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
    let [header_area, inner_area, footer_area] = vertical.areas(frame_size);

    let horizontal = Layout::horizontal([Min(0), Length(20)]);
    let [tabs_area, title_area] = horizontal.areas(header_area);

    let main = Layout::horizontal([Percentage(75), Percentage(25)]);
    let [left_area, right_area] = main.areas(inner_area);

    let area = match &cmp_area {
        LayoutArea::Header => header_area,
        LayoutArea::Tabs => tabs_area,
        LayoutArea::Title => title_area,
        LayoutArea::Inner => inner_area,
        LayoutArea::Left => left_area,
        LayoutArea::Right => right_area,
        LayoutArea::Footer => footer_area,
        LayoutArea::Hidden => Rect::default(),
    };
    debug!(target:"app", "calculated area for component '{}' - {:?}", cmp_name, area);

    area
}

#[memoized(key_expr = frame_size, store_type = HashMap<Rect, [Rect; 2]>)]
pub fn get_horizontal_area_split(frame_size: Rect) -> [Rect; 2] {
    let halfs = Layout::horizontal([Percentage(50), Percentage(50)]);
    halfs.areas(frame_size)
}

#[memoized(key_expr = (frame_size, max_height), store_type = HashMap<(Rect, u16), Rect>)]
pub fn get_terminal_area_max_height(frame_size: Rect, max_height: u16) -> Rect {
    if frame_size.height <= max_height {
        return frame_size;
    }
    let area = Rect::new(0, 0, frame_size.width, max_height);
    debug!(target:"app", "calculated terminal area max_height: {:?}",  area);

    area
}

#[cfg(test)]
mod tests {
    #![allow(unused_comparisons)]
    use super::*;

    #[test]
    fn test_get_component_area() {
        let frame_size = Rect::new(0, 0, 100, 100);
        let cmp_name = "test_component";

        let areas = [
            LayoutArea::Header,
            LayoutArea::Tabs,
            LayoutArea::Title,
            LayoutArea::Inner,
            LayoutArea::Left,
            LayoutArea::Right,
            LayoutArea::Footer,
        ];

        for &area in &areas {
            let result = get_component_area(frame_size, cmp_name, area);

            // Check that the result is a valid Rect
            assert!(result.x >= 0);
            assert!(result.y >= 0);
            assert!(result.width > 0);
            assert!(result.height > 0);
        }
    }
}
