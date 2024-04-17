use crate::{app::App, constants::*};
use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{Color, Modifier, Style, Stylize},
    symbols::Marker,
    widgets::{canvas::*, Block, Borders, List, ListState},
    Frame,
};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let horizontal = Layout::horizontal([Constraint::Percentage(75), Constraint::Percentage(25)]);
    let [left, right] = horizontal.areas(frame.size());
    // app.ukraine().set_size(left); // TODO: cannot borrow data in a `&` reference as mutable
    let list = List::new(app.ukraine().get_list_items())
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
    let mut list_state: ListState = app.ukraine().list_state();

    let map = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Map".light_blue())
                .title_alignment(Alignment::Center),
        )
        .marker(Marker::Braille)
        .x_bounds(app.ukraine().x_bounds())
        .y_bounds(app.ukraine().y_bounds())
        .paint(|ctx| {
            ctx.draw(app.ukraine());
        })
        .background_color(Color::Reset);

    // Render
    frame.render_widget(map, left);
    frame.render_stateful_widget(list, right, &mut list_state);

    // print to console canvas grid resolution
}
