use color_eyre::eyre::Result;
use ralertsinua_geo::*;
use ratatui::{
    prelude::*,
    widgets::{
        canvas::{Canvas, Painter, Shape},
        *,
    },
};
use rust_i18n::t;
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender;
use tracing::debug;

use super::{Component, Frame};
use crate::{action::Action, config::*, constants::*, layout::*};

#[allow(unused)]
const PADDING: f64 = 0.5;

#[derive(Debug)]
pub struct Map {
    command_tx: Option<UnboundedSender<Action>>,
    #[allow(unused)]
    config: Arc<dyn ConfigService>,
    // facade: Arc<dyn AlertsInUaFacade>,
    geo_client: Arc<dyn AlertsInUaGeo>,
    selected_region_idx: Option<usize>,
    selected_region: Option<Region>,
}

impl Map {
    pub fn new(config: Arc<dyn ConfigService>, geo_client: Arc<dyn AlertsInUaGeo>) -> Self {
        Self {
            command_tx: Option::default(),
            // facade,
            config,
            geo_client,
            selected_region_idx: None,
            selected_region: None,
        }
    }

    fn get_curr_area(&self, r: &Rect) -> Result<Rect> {
        let percent = 50;
        let idx = self.selected_region_idx;
        let curr_area = match idx.is_none() {
            false => {
                // INFO: https://ratatui.rs/how-to/layout/center-a-rect/
                let popup_layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Percentage((100 - percent) / 2),
                        Constraint::Percentage(percent),
                        Constraint::Percentage((100 - percent) / 2),
                    ])
                    .split(*r);

                Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Percentage((100 - percent) / 2),
                        Constraint::Percentage(percent),
                        Constraint::Percentage((100 - percent) / 2),
                    ])
                    .split(popup_layout[1])[1]
            }
            true => *r,
        };
        Ok(curr_area)
    }
}

/// Implement the Shape trait to draw map borders on canvas
impl Shape for Map {
    #[inline]
    fn draw(&self, painter: &mut Painter) {
        let selected_region = self.selected_region.clone();
        let borders = self.geo_client.borders();
        // If region was selected means we have last selected geo - then iterate region borders
        if selected_region.is_some() {
            // let b = self.geo_client.get_region_by(predicate);
            let borders = selected_region.unwrap().borders().unwrap();
        };
        borders.exterior().coords().for_each(|coord| {
            if let Some((x, y)) = painter.get_point(coord.x, coord.y) {
                painter.paint(x, y, *MARKER_COLOR);
            }
        });
    }
}

impl Component for Map {
    fn display(&self) -> Result<String> {
        let regions = self.geo_client.regions();
        debug!("Map->regions: len: {}", regions.len());
        Ok("Map".to_string())
    }

    fn placement(&self) -> LayoutPoint {
        LayoutPoint(LayoutArea::Left, Some(LayoutTab::Tab1))
    }

    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Tick => {}
            Action::Selected(idx) => {
                if idx.is_some() {
                    self.selected_region_idx = idx;
                    let selected_i = idx.unwrap();
                    self.selected_region =
                        self.geo_client.regions().get(selected_i).cloned();
                } else {
                    self.selected_region_idx = None;
                    self.selected_region = None;
                }
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: &Rect) -> Result<()> {
        let (x_bounds, y_bounds) = if self.selected_region_idx.is_some() {
            self.selected_region.clone().unwrap().get_x_y_bounds()
        } else {
            self.geo_client.get_x_y_bounds()
        };

        // let (x_bounds, y_bounds) = self.geo_client.get_x_y_bounds();

        let area = self.get_curr_area(area)?;
        let widget = Canvas::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(t!("views.Map.title").to_string().light_blue())
                    .title_alignment(Alignment::Center),
            )
            .marker(Marker::Braille)
            .x_bounds(x_bounds)
            .y_bounds(y_bounds)
            .paint(|ctx| {
                ctx.draw(self);
            })
            .background_color(Color::Reset);
        f.render_widget(widget, area);
        Ok(())
    }
}

/* #[cfg(test)]
mod tests {
    use super::*;
    use geo::HasDimensions;

    #[test]
    fn test_map_new() {
        let map = Map::new(Ukraine::new_arc(), Arc::new(Config::init().unwrap()));
        assert!(map.command_tx.is_none());
        assert!(!map.map.borders().is_empty());
        assert!(map.ukraine.read().unwrap().regions().is_empty());
        // match map.borders.try_from()
    }
} */
