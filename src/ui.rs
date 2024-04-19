use crate::{app::App, constants::*};
use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{Color, Modifier, Style, Stylize},
    symbols::Marker,
    widgets::{canvas::*, Block, Borders, List},
    Frame,
};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let ukraine = app.ukraine_mut();
    let horizontal = Layout::horizontal([Constraint::Percentage(75), Constraint::Percentage(25)]);
    let [left, right] = horizontal.areas(frame.size());

    ukraine.set_size(left);

    // In addition to `List::new`, any iterator whose element is convertible to `ListItem` can be collected into `List`.
    let list = List::new(ukraine.get_list_items().clone())
        .block(
            Block::bordered()
                .title("Regions")
                .title_alignment(Alignment::Center),
        )
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(SELECTED_STYLE_FG),
        )
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
        .x_bounds(ukraine.x_bounds())
        .y_bounds(ukraine.y_bounds())
        .paint(|ctx| {
            ctx.draw(ukraine);
        })
        .background_color(Color::Reset);

    // Render
    frame.render_widget(map, left);
    frame.render_stateful_widget(list, right, &mut ukraine.list_state().clone());

    // print to console canvas grid resolution
}
