use super::{Component, Frame};
use crate::{
    action::Action,
    config::get_config_prop,
    constants::*,
    tui::LayoutArea,
    ukraine::*,
};
use color_eyre::eyre::Result;
use geo::{Geometry, HasDimensions, Polygon};
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
const UKRAINE_BORDERS_POYGON_WKT: &'static str = include_str!("../../.data/ukraine.wkt");
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
    ukraine: Arc<RwLock<Ukraine>>,
    borders: Polygon,
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
    pub fn new(ukraine: Arc<RwLock<Ukraine>>) -> Self {
        use std::str::FromStr;
        let borders: Polygon = Wkt::from_str(UKRAINE_BORDERS_POYGON_WKT)
            .unwrap()
            .try_into()
            .unwrap();
        Self {
            command_tx: Option::default(),
            borders,
            ukraine,
        }
    }
}

/// Implement the Shape trait to draw map borders on canvas
impl Shape for Map {
    #[tracing::instrument]
    #[inline]
    fn draw(&self, painter: &mut Painter) {
        let coords_iter = self.borders.exterior().coords().into_iter();
        coords_iter.for_each(|&coord| {
            if let Some((x, y)) = painter.get_point(coord.x, coord.y) {
                painter.paint(x, y, MARKER_COLOR.clone());
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
            Action::Selected(i) => {
                info!("Map->update Action::Selected: {:?}", i);
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        let widget = Canvas::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(t!("views.Map.title").to_string().light_blue())
                    .title_alignment(Alignment::Center),
            )
            .marker(Marker::Braille)
            .x_bounds(Self::x_bounds())
            .y_bounds(Self::y_bounds())
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
        let map = Map::new(Ukraine::new_arc());
        assert!(map.command_tx.is_none());
        assert!(map.borders.is_empty() == false);
        assert!(map.ukraine.read().unwrap().regions().is_empty() == true);
        // match map.borders.try_from()
    }
}
