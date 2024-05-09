use color_eyre::eyre::Result;
use geo::{Centroid, Coord};
use ralertsinua_geo::*;
use ratatui::{
    prelude::*,
    widgets::{
        canvas::{Canvas, Context, Painter, Shape},
        // canvas::{Grid, Layer}, // FIXME: how to use directly?
        *,
    },
};
use rust_i18n::t;
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender;
#[allow(unused)]
use tracing::debug;

use super::{Component, Frame, WithPlacement};
use crate::{action::Action, config::*, constants::*, layout::*};

#[allow(unused)]
const PADDING: f64 = 0.5;

#[derive(Debug)]
pub struct Map<'a> {
    command_tx: Option<UnboundedSender<Action>>,
    placement: LayoutPoint,
    #[allow(unused)]
    title: Line<'a>,
    #[allow(unused)]
    config: Arc<dyn ConfigService>,
    geo_client: Arc<dyn AlertsInUaGeo>,
    selected_location: Option<Location>,
    // context: Context<'a>,
}

impl<'a> Map<'a> {
    pub fn new(config: Arc<dyn ConfigService>, geo_client: Arc<dyn AlertsInUaGeo>) -> Self {
        let context = Context::new(0, 0, [0.0, 0.0], [0.0, 0.0], Marker::Braille);
        // let grid = context.grid();
        Self {
            command_tx: Option::default(),
            placement: LayoutPoint(LayoutArea::Left, Some(LayoutTab::Tab1)),
            title: Line::default(),
            config,
            geo_client,
            selected_location: None,
            // context,
        }
    }
}

impl WithPlacement for Map<'_> {
    fn placement(&self) -> &LayoutPoint {
        &self.placement
    }
}

/// Implement the Shape trait to draw map boundary on canvas
impl<'a> Shape for Map<'a> {
    #[inline]
    fn draw(&self, painter: &mut Painter) {
        let boundary = self.geo_client.boundary();
        // If location was selected means we have last selected geo - then iterate location boundary
        let boundary = match &self.selected_location {
            Some(location) => location.boundary(),
            None => self.geo_client.boundary(),
        };
        boundary.exterior().coords().for_each(|coord| {
            if let Some((x, y)) = painter.get_point(coord.x, coord.y) {
                painter.paint(x, y, *MARKER_COLOR);
            }
        });
        // Example mark center of the location
        self.geo_client
            .locations()
            .iter()
            .filter(|l| l.location_type == *"state")
            .for_each(|location| {
                let center: Coord = location.geometry.centroid().unwrap().into();
                if let Some((x, y)) = painter.get_point(center.x, center.y) {
                    painter.paint(x, y, *ALERT_ROW_COLOR);
                }
            });
    }
}

impl<'a> Component<'a> for Map<'a> {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Tick => {}
            Action::Selected(a) => match a {
                Some(a) => {
                    self.selected_location =
                        self.geo_client.get_location_by_uid(a.location_uid).cloned()
                }
                None => {
                    self.selected_location = None;
                }
            },
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame) -> Result<()> {
        let area = self.get_area(f.size())?;
        let (x_bounds, y_bounds) = self.geo_client.get_x_y_bounds();
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
                ctx.layer();
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
        assert!(!map.map.boundary().is_empty());
        assert!(map.ukraine.read().unwrap().locations().is_empty());
        // match map.boundary.try_from()
    }
} */
