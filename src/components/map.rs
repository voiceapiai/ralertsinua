use super::{Component, Frame};
use crate::{action::Action, config::Config, constants::*, ukraine::Ukraine};
use color_eyre::eyre::Result;
use geo::Polygon;
// use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::*,
    widgets::{
        canvas::{Canvas, Painter, Shape},
        *,
    },
};
// use serde::{Deserialize, Serialize};
#[allow(unused)]
use std::{collections::HashMap, time::Duration};
use tokio::sync::mpsc::UnboundedSender;

#[derive(Debug)]
pub struct Map {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    borders: Polygon,
}

impl Map {
    pub fn new() -> Self {
        use std::str::FromStr;
        use wkt::Wkt;
        let borders: Polygon = Wkt::from_str(&UKRAINE_BORDERS_WKT)
            .unwrap()
            .try_into()
            .unwrap();
        Self {
            command_tx: Option::default(),
            config: Config::default(),
            borders,
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

    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Tick => {}
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        let map = Canvas::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Map".light_blue())
                    .title_alignment(Alignment::Center),
            )
            .marker(Marker::Braille)
            .x_bounds(Ukraine::x_bounds())
            .y_bounds(Ukraine::y_bounds())
            .paint(|ctx| {
                ctx.draw(self);
            })
            .background_color(Color::Reset);
        f.render_widget(map, area);
        Ok(())
    }
}
