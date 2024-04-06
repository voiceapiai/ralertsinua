use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{Color, Modifier, Style, Stylize},
    symbols::Marker,
    widgets::{canvas::*, Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::{App, Ukraine};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let horizontal = Layout::horizontal([Constraint::Percentage(75), Constraint::Percentage(25)]);
    let [map, right] = horizontal.areas(frame.size());
    let records: Vec<ListItem> = app
        .records
        .iter()
        .map(|r| ListItem::new(format!("{} - {}", r.id, r.name)))
        .collect();

    frame.render_widget(
        List::new(records)
            .block(
                Block::bordered()
                    .title("Ukraine")
                    .title_alignment(Alignment::Center)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Cyan))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true),
        right,
    );
    frame.render_widget(
        Canvas::default()
            .block(Block::default().borders(Borders::ALL).title("World"))
            .marker(Marker::Braille)
            .paint(|ctx| {
                ctx.draw(&Ukraine {
                    color: Color::Green,
                    data: Ukraine::default().data,
                });
                ctx.print(10.0, 10.0, "You are here".yellow());
            })
            .x_bounds([-180.0, 180.0])
            .y_bounds([-90.0, 90.0]),
        map,
    );
}
