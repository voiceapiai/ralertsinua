use crate::constants::*;
#[allow(unused)]
use anyhow::*;
use arrayvec::ArrayVec;
// use delegate::delegate;
#[allow(unused)]
use either::Either;
use geo::{Coord, Polygon};
use getset::{Getters, MutGetters};
use ratatui::{
    layout::Rect,
    prelude::*,
    widgets::canvas::{Painter, Shape},
    widgets::{ListItem, ListState},
};
use serde::*;
use strum::{Display, EnumString};
use tracing::info;

// use geo::algorithm::bounding_rect::BoundingRect;
// use geo::algorithm::simplify_vw::SimplifyVw;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, Display)]
pub enum AlertStatus {
    /// Active
    A,
    /// Partially active
    P,
    /// No information
    N,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Region {
    pub id: i8,
    pub a_id: i8,
    pub osm_id: i64,
    pub geo: String,
    pub name: String,
    pub name_en: String,
    #[sqlx(rename = "status", default)]
    pub status: Option<String>,
}
pub type RegionArrayVec = ArrayVec<Region, 27>;
pub type RegionListVec<'a> = ArrayVec<ListItem<'a>, 27>;

impl Region {
    #[allow(unused)]
    fn to_list_item(&self, index: usize) -> ListItem {
        // let bg_color = match index % 2 { 0 => NORMAL_ROW_COLOR, _ => ALERT_ROW_COLOR, };
        let line = if ((self.status.as_deref() == Some("A")) || index == 0) {
            Line::styled(format!(" ⊙ {}", self.name), ALERT_ROW_COLOR)
        } else {
            Line::styled(format!("  {}", self.name), (TEXT_COLOR))
        };

        ListItem::new(line)
    }
}

#[derive(Debug, Default, Getters, MutGetters)]
pub struct Ukraine {
    borders: String,
    #[getset(get = "pub", get_mut)]
    regions: RegionArrayVec,
    pub center: Coord,
    #[getset(get = "pub", set = "pub")]
    size: Rect,
    #[getset(get = "pub", get_mut = "pub")]
    list: RegionListVec<'static>,
    // #[getset(get = "pub")]
    list_state: ListState,
    #[getset(get = "pub", get_mut)]
    last_selected: Option<usize>,
}

impl Ukraine {
    pub fn new(borders: String, regions: RegionArrayVec) -> Self {
        let center = Coord::from(CENTER);
        let bbox = Rect::default();
        let list = ArrayVec::<ListItem, 27>::new();
        /* let mut list = ArrayVec::<ListItem, 27>::new();
        regions.iter().enumerate().for_each(|(i, r)| {
            let item = r.to_list_item(i);
            list.push(item);
        }); */
        let list_state = ListState::default();
        let last_selected = None;

        Self {
            borders,
            regions,
            center,
            size: bbox,
            list,
            list_state,
            last_selected,
        }
    }

    /* delegate! {
        to self.list {
            pub fn next(&mut self);
            pub fn previous(&mut self);
            pub fn unselect(&mut self);
        }
    } */

    pub fn borders(&self) -> Polygon {
        use std::str::FromStr;
        use wkt::Wkt;
        let geom: Polygon = Wkt::from_str(&self.borders).unwrap().try_into().unwrap();
        geom
    }

    pub fn get_list_items(&self) -> RegionListVec {
        let mut list = ArrayVec::<ListItem, 27>::new();
        self.regions.iter().enumerate().for_each(|(i, r)| {
            let item = r.to_list_item(i);
            list.push(item);
        });
        list
    }

    pub fn list_state(&self) -> ListState {
        self.list_state.clone()
    }

    #[inline]
    pub fn boundingbox(&self) -> [(f64, f64); 2] {
        #[allow(unused_parens)]
        (BOUNDINGBOX)
    }

    #[inline]
    pub fn x_bounds(&self) -> [f64; 2] {
        [
            self.boundingbox().first().unwrap().0 - PADDING,
            self.boundingbox().last().unwrap().0 + PADDING,
        ]
    }

    #[inline]
    pub fn y_bounds(&self) -> [f64; 2] {
        [
            self.boundingbox().first().unwrap().1 - PADDING,
            self.boundingbox().last().unwrap().1 + PADDING,
        ]
    }

    /// Store size of the terminal rect
    #[inline]
    pub fn set_size(&mut self, rect: Rect) {
        self.size = rect;
    }
    /// Get the resolution of the grid in number of dots
    #[inline]
    pub fn resolution(&self) -> (f64, f64) {
        (
            f64::from(self.size.width) * 2.0,
            f64::from(self.size.height) * 4.0,
        )
    }

    #[tracing::instrument]
    pub fn next(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.regions.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => self.last_selected.unwrap_or(0),
        };
        self.list_state.select(Some(i));
        info!("List->next, selected region: {:?}", self.regions[i]);
    }

    #[tracing::instrument]
    pub fn previous(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.regions.len() - 1
                } else {
                    i - 1
                }
            }
            None => self.last_selected.unwrap_or(0),
        };
        self.list_state.select(Some(i));
        info!("List->previous, selected region: {:?}", self.regions[i]);
    }

    pub fn unselect(&mut self) {
        let offset = self.list_state.offset();
        self.last_selected = self.list_state.selected();
        self.list_state.select(None);
        *self.list_state.offset_mut() = offset;
    }

    pub fn go_top(&mut self) {
        self.list_state.select(Some(0));
    }

    pub fn go_bottom(&mut self) {
        self.list_state.select(Some(self.regions.len() - 1));
    }
}

impl Shape for Ukraine {
    /// Implement the Shape trait for Ukraine to draw map borders on canvas
    #[tracing::instrument]
    #[inline]
    fn draw(&self, painter: &mut Painter) {
        let borders = self.borders();
        let coords_iter = borders.exterior().coords().into_iter();
        coords_iter.for_each(|&coord| {
            if let Some((x, y)) = painter.get_point(coord.x, coord.y) {
                painter.paint(x, y, MARKER_COLOR);
            }
        });
        // TODO: mark center - not working
        if let Some((cx, cy)) = painter.get_point(self.center.x, self.center.y) {
            painter.paint(cx, cy, MARKER_COLOR);
        }
    }
}
