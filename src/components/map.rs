use super::{Component, Frame};
use crate::{
    action::Action,
    config::*,
    constants::*,
    tui::LayoutArea,
    ukraine::{self, *},
};
use color_eyre::eyre::Result;
use geo::{BoundingRect, CoordsIter, Geometry, HasDimensions, LineString, Polygon};
use ratatui::{
    prelude::*,
    widgets::{
        canvas::{Canvas, Painter, Shape},
        *,
    },
};
use rust_i18n::t;
use strum::Display;
use tracing::info;
use wkt::*;
// use serde::{Deserialize, Serialize};
#[allow(unused)]
use std::{collections::HashMap, time::Duration};
use tokio::sync::mpsc::UnboundedSender;

/// Ukraine borders represented as Polygon in WKT file
const UKRAINE_BORDERS_POYGON_WKT: &str = include_str!("../../.data/ukraine.wkt");
/// Ukraine bounding box coords tuple - (min_x, min_y), (max_x, max_y)
///
/// <em>Територія України розташована між 44°23' і 52°25' північної широти та між 22°08' і 40°13' східної довготи</em>
const UKRAINE_BBOX: [(f64, f64); 2] = [(22.08, 44.23), (40.13, 52.25)];
/// Ukraine center
///
/// <em>Центр України знаходиться в точці з географічними координатами `49°01'` північної широти і `31°02'` східної довготи. Ця точка розміщена за 2 км на захід від м. Ватутіного у Черкаській області – с. Мар'янівка. За іншою версією – с. Добровеличківка Кіровоградської області.</em>
#[allow(unused)]
const UKRAINE_CENTER: (f64, f64) = (49.01, 31.02);
const PADDING: f64 = 0.5;

#[derive(Debug)]
pub struct Map {
    command_tx: Option<UnboundedSender<Action>>,
    #[allow(unused)]
    config: Arc<dyn ConfigService>,
    ukraine: Arc<RwLock<Ukraine>>,
    borders: Polygon,
    last_selected: Option<usize>,
    last_selected_geo: Option<String>,
}

trait MapBounds {
    fn boundingbox() -> [(f64, f64); 2];
    fn x_bounds() -> [f64; 2];
    fn y_bounds() -> [f64; 2];
}

impl MapBounds for Map {
    #[inline]
    fn boundingbox() -> [(f64, f64); 2] {
        UKRAINE_BBOX
    }

    #[inline]
    fn x_bounds() -> [f64; 2] {
        [
            UKRAINE_BBOX.first().unwrap().0 - PADDING,
            UKRAINE_BBOX.last().unwrap().0 + PADDING,
        ]
    }

    #[inline]
    fn y_bounds() -> [f64; 2] {
        [
            UKRAINE_BBOX.first().unwrap().1 - PADDING,
            UKRAINE_BBOX.last().unwrap().1 + PADDING,
        ]
    }
}

impl Map {
    pub fn new(ukraine: Arc<RwLock<Ukraine>>, config: Arc<dyn ConfigService>) -> Self {
        use std::str::FromStr;
        let borders: Polygon = Wkt::from_str(UKRAINE_BORDERS_POYGON_WKT)
            .unwrap()
            .try_into()
            .unwrap();
        Self {
            command_tx: Option::default(),
            borders,
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

    fn get_x_y_bounds(&self) -> Result<([f64; 2], [f64; 2])> {
        let lsg = self.get_last_selected_geo();
        let x_y_bounds = match lsg.is_empty() {
            false => {
                use std::str::FromStr;
                let geom: Geometry = Wkt::from_str(lsg).unwrap().try_into().unwrap();
                let b = geom.bounding_rect().unwrap();
                (
                    [b.min().x - 0.0, b.max().x + 0.0],
                    [b.min().y - 0.0, b.max().y + 0.0],
                )
            }
            true => (Self::x_bounds(), Self::y_bounds()),
        };
        Ok(x_y_bounds)
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
        let coords_iter = self.borders.exterior().coords();
        // If region was selected means we have last selected geo - then iterate region borders
        if !lsg.is_empty() {
            use std::str::FromStr;
            let geom: Geometry = Wkt::from_str(lsg).unwrap().try_into().unwrap();
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
        let (x_bounds, y_bounds) = self.get_x_y_bounds()?;
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

    #[test]
    fn test_map_new() {
        let map = Map::new(Ukraine::new_arc(), Arc::new(Config::init().unwrap()));
        assert!(map.command_tx.is_none());
        assert!(!map.borders.is_empty());
        assert!(map.ukraine.read().unwrap().regions().is_empty());
        // match map.borders.try_from()
    }
}
