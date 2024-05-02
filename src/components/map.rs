use super::{Component, Frame};
use crate::{action::Action, config::*, constants::*, tui::LayoutArea, ukraine::*};
use color_eyre::eyre::Result;
use geo::Geometry;
use ralertsinua_geo::*;
use ratatui::{
    prelude::*,
    widgets::{
        canvas::{Canvas, Painter, Shape},
        *,
    },
};
use rust_i18n::t;
#[allow(unused)]
use std::{collections::HashMap, time::Duration};
use tokio::sync::mpsc::UnboundedSender;
use tracing::info;
#[allow(unused)]
const PADDING: f64 = 0.5;

#[derive(Debug)]
pub struct Map {
    command_tx: Option<UnboundedSender<Action>>,
    #[allow(unused)]
    config: Arc<dyn ConfigService>,
    ukraine: Arc<RwLock<Ukraine>>,
    map: Arc<dyn AlertsInUaMap>,
    last_selected: Option<usize>,
    last_selected_geo: Option<String>,
}

impl Map {
    pub fn new(ukraine: Arc<RwLock<Ukraine>>, config: Arc<dyn ConfigService>) -> Self {
        let map: Arc<dyn AlertsInUaMap> = Arc::new(AlertsInUaMapBounds::default());
        Self {
            command_tx: Option::default(),
            map,
            ukraine,
            config,
            last_selected: None,
            last_selected_geo: None,
        }
    }

    fn get_last_selected_geo(&self) -> &str {
        match self.last_selected_geo {
            Some(ref lsg) => lsg.as_str(),
            None => "",
        }
    }

    fn get_curr_area(&self, r: Rect) -> Result<Rect> {
        let percent = 50;
        let lsg = self.get_last_selected_geo();
        let curr_area = match lsg.is_empty() {
            false => {
                // INFO: https://ratatui.rs/how-to/layout/center-a-rect/
                let popup_layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Percentage((100 - percent) / 2),
                        Constraint::Percentage(percent),
                        Constraint::Percentage((100 - percent) / 2),
                    ])
                    .split(r);

                Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Percentage((100 - percent) / 2),
                        Constraint::Percentage(percent),
                        Constraint::Percentage((100 - percent) / 2),
                    ])
                    .split(popup_layout[1])[1]
            }
            true => r,
        };
        Ok(curr_area)
    }
}

/// Implement the Shape trait to draw map borders on canvas
impl Shape for Map {
    #[tracing::instrument]
    #[inline]
    fn draw(&self, painter: &mut Painter) {
        let lsg = self.get_last_selected_geo();
        let coords_iter = self.map.borders().exterior().coords();
        // If region was selected means we have last selected geo - then iterate region borders
        if !lsg.is_empty() {
            let geom: Geometry = from_wkt_to_geom(lsg).unwrap();
            match geom {
                Geometry::Polygon(poly) => {
                    let coords_iter = poly.exterior().coords();
                }
                Geometry::MultiPolygon(multi_poly) => {
                    // If you want to handle only the first polygon in a MultiPolygon
                    if let Some(poly) = multi_poly.0.first() {
                        let coords_iter = poly.exterior().coords();
                    }
                }
                _ => {
                    // Handle other geometry types if necessary
                }
            }
        };
        coords_iter.for_each(|coord| {
            if let Some((x, y)) = painter.get_point(coord.x, coord.y) {
                painter.paint(x, y, *MARKER_COLOR);
            }
        });
    }
}

impl Component for Map {
    fn display(&mut self) -> Result<String> {
        Ok("Map".to_string())
    }

    fn placement(&mut self) -> LayoutArea {
        LayoutArea::Left_75
    }

    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Tick => {}
            Action::Selected(selected) => {
                if selected.is_some() {
                    self.last_selected = selected;
                    let selected_i = selected.unwrap();
                    let ukraine = self.ukraine.read().unwrap();
                    let selected_region = ukraine.regions().get(selected_i).unwrap();
                    info!("Map->update Action::Selected: {:?}", selected_region);
                } else {
                    self.last_selected = None;
                    self.last_selected_geo = None;
                }
            }
            Action::SetRegionGeo(geo) => {
                self.last_selected_geo = Some(geo);
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        let (x_bounds, y_bounds) = self
            .map
            .get_x_y_bounds(self.last_selected_geo.clone())
            .unwrap();
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

#[cfg(test)]
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
}
