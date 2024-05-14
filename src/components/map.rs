use std::fmt::Debug;

use geo::Rect as GeoRect;
use ralertsinua_geo::*;
use ralertsinua_models::*;
use ratatui::{
    prelude::*,
    widgets::{
        canvas::{Canvas, Context},
        *,
    },
};
use rust_i18n::t;
use tokio::sync::mpsc::UnboundedSender;
#[allow(unused)]
use tracing::debug;
// use tui_popup::Popup;

use super::{Component, Frame, Result, WithPlacement};
use crate::{action::*, config::*, layout::*, tui_helpers::*};

#[derive(Debug)]
pub struct Map<'a> {
    command_tx: Option<UnboundedSender<Action>>,
    placement: LayoutPoint,
    #[allow(unused)]
    title: Line<'a>,
    #[allow(unused)]
    config: Config,
    bounding_rect: GeoRect,
    boundary: CountryBoundary,
    locations: [Location; 27],
    selected_location_uid: i32,
    oblast_statuses: AirRaidAlertOblastStatuses,
    //
    width: u16,
    height: u16,
    resolution: (f64, f64),
}

impl<'a> Map<'a> {
    #[inline]
    pub fn new() -> Self {
        let context = Context::new(0, 0, [0.0, 0.0], [0.0, 0.0], Marker::Braille);
        Self {
            command_tx: Option::default(),
            placement: LayoutPoint(LayoutArea::Left, Some(LayoutTab::Tab1)),
            title: Line::default(),
            config: Config::default(),
            boundary: CountryBoundary::default(),
            bounding_rect: *UKRAINE_BBOX,
            locations: core::array::from_fn(|_| Location::default()),
            selected_location_uid: -1,
            oblast_statuses: AirRaidAlertOblastStatuses::default(),
            //
            width: 0,
            height: 0,
            resolution: (0.0, 0.0),
        }
    }

    #[inline]
    pub fn set_grid_size(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        self.resolution = (f64::from(width) * 2.0, f64::from(height) * 4.0);
        debug!(target:"app", "Map grid size: width: {}, height: {}, x_Y_bounds: {:?}, resolution: {:?}", width, height, self.get_x_y_bounds(), self.resolution);
    }

    #[inline]
    pub fn get_location_by<P>(&self, mut predicate: P) -> Option<Location>
    where
        P: FnMut(&Location) -> bool,
    {
        self.locations.iter().find(|r| predicate(r)).cloned()
    }

    #[inline]
    pub fn get_selected_location(&mut self) -> Option<Location> {
        self.get_location_by(|l| l.location_uid == self.selected_location_uid)
    }

    #[inline]
    pub fn get_selected_alert_status(&mut self) -> Option<AirRaidAlertOblastStatus> {
        self.oblast_statuses
            .iter()
            .find(|&os| os.location_uid == self.selected_location_uid)
            .cloned()
    }
}

impl WithPlacement<'_> for Map<'_> {
    #[inline]
    fn placement(&self) -> &LayoutPoint {
        &self.placement
    }
}

impl WithBoundingRect for Map<'_> {
    #[inline]
    fn bounding_rect(&self) -> geo::Rect {
        self.bounding_rect
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

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
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
            Action::GetAirRaidAlertOblastStatuses(data) => {
                self.oblast_statuses = data;
            }
            Action::SelectLocationByUid(a) => match a {
                Some(location_uid) => {
                    self.selected_location_uid = location_uid as i32;
                    debug!(target:"app", "Map: selected_location_uid: {}", location_uid);
                }
                None => {
                    self.selected_location_uid = -1;
                }
            },
            Action::Online(online) => {
                self.title = get_title_with_online_status(
                    t!("views.Map.title"),
                    self.config.online(),
                )
                .alignment(Alignment::Left);
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame) -> Result<()> {
        let size: Rect = f.size();
        let area: Rect = self.get_area(size)?;
        let (x_bounds, y_bounds) = self.get_x_y_bounds();
        let selected_location = self.get_selected_location();
        let selected_alert = self.get_selected_alert_status();
        let title = self.title.clone();
        let widget = Canvas::default()
            .block(Block::bordered().title(title))
            .marker(Marker::Braille)
            .x_bounds(x_bounds)
            .y_bounds(y_bounds)
            .paint(move |ctx| {
                //  Draw country borders with ctx
                ctx.draw(&self.boundary);

                // Draw & Print selected location with ctx
                self.locations.iter().for_each(|l| {
                    // Draw location
                    ctx.draw(l);
                    // Print location name
                    let (x, y) = l.center();
                    let text = l
                        .get_name_by_locale(self.config.get_locale())
                        .split(' ')
                        .next()
                        .unwrap_or("");
                    let status: &AlertStatus = self
                        .oblast_statuses
                        .iter()
                        .find(|&os| os.location_uid == l.location_uid)
                        .unwrap()
                        .status();
                    let is_selected = (l.location_uid) == self.selected_location_uid;
                    let line = get_styled_line_icon_by_status(status, &is_selected);
                    ctx.print(x, y, line);
                });
            })
            .background_color(Color::Reset);
        f.render_widget(widget, area);

        // popup
        if let Some(sa) = selected_alert {
            let popup_area = bottom_left_rect(area, 30, 20);
            let popup_bg = get_color_by_status(sa.status());
            let popup_text =
                Text::from(format!("{}\n{}", sa.location_title_en(), sa.status()));
            // let popup = Popup::new("Alert Status Details:", popup_text)
            //     .style(Style::new().bg(popup_bg));
            // f.render_widget(&popup, popup_area);
            let paragraph = Paragraph::new(popup_text)
                .dark_gray()
                .alignment(Alignment::Center);
            let block = Block::bordered()
                .bg(popup_bg)
                .title("Alert Status Details:".white().bold().italic());
            f.render_widget(paragraph.block(block), popup_area);
        };
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
