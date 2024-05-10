// use cached::proc_macro::cached;
use color_eyre::eyre::Result;
use geo::{Centroid, Coord, Polygon};
use ralertsinua_geo::*;
use ratatui::{
    prelude::*,
    widgets::{
        canvas::{Canvas, Context, Painter, Shape},
        *,
    },
};
use rust_i18n::t;
use tokio::sync::mpsc::UnboundedSender;
#[allow(unused)]
use tracing::debug;

use super::{Component, Frame, WithPlacement};
use crate::{action::Action, config::*, constants::*, layout::*};

fn get_rec_center(rec: &Rect) -> (f64, f64) {
    let x = f64::from(rec.width) / 2.0;
    let y = f64::from(rec.height) / 2.0;
    (x, y)
}

#[derive(Debug)]
pub struct Map<'a> {
    command_tx: Option<UnboundedSender<Action>>,
    placement: LayoutPoint,
    #[allow(unused)]
    title: Line<'a>,
    #[allow(unused)]
    config: Config,
    #[allow(unused)]
    boundary: Polygon,
    locations: [Location; 27],
    selected_location: Option<Location>,
    //
    width: u16,
    height: u16,
    x_bounds: [f64; 2],
    y_bounds: [f64; 2],
    resolution: (f64, f64),
}

impl<'a> Map<'a> {
    pub fn new() -> Self {
        let context = Context::new(0, 0, [0.0, 0.0], [0.0, 0.0], Marker::Braille);
        // let grid = context.grid();
        Self {
            command_tx: Option::default(),
            placement: LayoutPoint(LayoutArea::Left, Some(LayoutTab::Tab1)),
            title: Line::default(),
            config: Config::default(),
            boundary: default_polygon(),
            locations: core::array::from_fn(|_| Location::default()),
            selected_location: None,
            //
            width: 0,
            height: 0,
            x_bounds: [UKRAINE_BBOX[0].0, UKRAINE_BBOX[0].1],
            y_bounds: [UKRAINE_BBOX[1].0, UKRAINE_BBOX[1].1],
            resolution: (0.0, 0.0),
        }
    }

    pub fn set_grid_size(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        self.resolution = (f64::from(width) * 2.0, f64::from(height) * 4.0);
        debug!(target:"app", "Map grid size: width: {}, height: {}, x_bounds: {:?}, y_bounds: {:?}, resolution: {:?}", width, height, self.x_bounds, self.y_bounds, self.resolution);
    }

    /// Convert the `(x, y)` coordinates to location of a point on the grid
    pub fn get_point(&self, x: f64, y: f64) -> Option<(usize, usize)> {
        let left = self.x_bounds[0];
        let right = self.x_bounds[1];
        let top = self.y_bounds[1];
        let bottom = self.y_bounds[0];
        if x < left || x > right || y < bottom || y > top {
            return None;
        }
        let width = (self.x_bounds[1] - self.x_bounds[0]).abs();
        let height = (self.y_bounds[1] - self.y_bounds[0]).abs();
        if width == 0.0 || height == 0.0 {
            return None;
        }
        let x = ((x - left) * (self.resolution.0 - 1.0) / width) as usize;
        let y = ((top - y) * (self.resolution.1 - 1.0) / height) as usize;
        Some((x, y))
    }

    pub fn get_location_by<P>(&self, mut predicate: P) -> Option<Location>
    where
        P: FnMut(&Location) -> bool,
    {
        self.locations.iter().find(|r| predicate(r)).cloned()
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
        // If location was selected means we have last selected geo - then iterate location boundary
        let boundary = match &self.selected_location {
            Some(location) => location.boundary(),
            None => &self.boundary,
        };
        self.boundary.exterior().coords().for_each(|coord| {
            if let Some((x, y)) = painter.get_point(coord.x, coord.y) {
                painter.paint(x, y, *MARKER_COLOR);
            }
        });

        // Example mark center of the location
        self.locations
            .iter()
            .filter(|l| l.location_type == *"state")
            .for_each(|l| {
                let center: Coord = l.geometry().centroid().unwrap().into();
                if let Some((x, y)) = painter.get_point(center.x, center.y) {
                    painter.paint(x, y, Color::LightBlue);
                }
            });
    }
}

impl<'a> Component<'a> for Map<'a> {
    fn init(&mut self, r: Rect) -> Result<()> {
        self.set_grid_size(r.width, r.height);
        Ok(())
    }

    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Tick => {}
            Action::Resize(width, heith) => self.set_grid_size(width, heith),
            Action::GetBoundaries(boundary) => {
                self.boundary = boundary;
            }
            Action::GetLocations(locations) => {
                self.locations = locations;
            }
            Action::SelectLocation(a) => match a {
                Some(location_uid) => {
                    self.selected_location =
                        self.get_location_by(|r| r.location_uid == location_uid as i32);
                    debug!(target:"app", "Map: selected_location_uid: {}", location_uid);
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
        let size: Rect = f.size();
        let area: Rect = self.get_area(size)?;
        let widget = Canvas::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(t!("views.Map.title").to_string().light_blue())
                    .title_alignment(Alignment::Center),
            )
            .marker(Marker::Braille)
            .x_bounds(self.x_bounds)
            .y_bounds(self.y_bounds)
            .paint(|ctx| {
                ctx.draw(self);
                // Example mark center of the location
                let (x, y) = get_rec_center(&area);
                ctx.print(x, y, "Ukraine")
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
