use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{Color, Modifier, Style, Stylize},
    symbols::Marker,
    widgets::{canvas::*, Block, Borders, List, ListItem},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let horizontal = Layout::horizontal([Constraint::Percentage(75), Constraint::Percentage(25)]);
    let [left, right] = horizontal.areas(frame.size());
    app.ukraine.set_size(left);
    let regions: Vec<ListItem> = app
        .ukraine
        .regions()
        .map(|r| ListItem::new(format!("{} - {}", r.id, r.name)))
        .collect();
    let regions_list = List::new(regions)
        .block(
            Block::bordered()
                .title("Regions")
                .title_alignment(Alignment::Center),
        )
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true);

    let map = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Map".light_blue())
                .title_alignment(Alignment::Center),
        )
        .marker(Marker::Braille)
        .x_bounds(app.ukraine.x_bounds())
        .y_bounds(app.ukraine.y_bounds())
        .paint(|ctx| {
            ctx.draw(&app.ukraine);
        })
        .background_color(Color::Reset);

    // Render
    frame.render_widget(regions_list, right);
    frame.render_widget(map, left);

    // print to console canvas grid resolution
}
