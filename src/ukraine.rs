use crate::{alerts::*, constants::*};
#[allow(unused)]
use anyhow::*;
use arrayvec::ArrayVec;
use delegate::delegate;
use geo::{Coord, Polygon};
use getset::{Getters, MutGetters, Setters};
use ratatui::{
    layout::Rect,
    prelude::*,
    widgets::{ListItem, ListState},
};
use serde::*;
use tracing::info;

// use geo::algorithm::bounding_rect::BoundingRect;
// use geo::algorithm::simplify_vw::SimplifyVw;

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
    pub fn to_list_item(&self, index: i8, alert_status: char) -> ListItem<'static> {
        let symbol = "🜸"; //  "🔥";
        let line = match alert_status {
            'A' => Line::styled(
                format!("{}) {} {}", index, self.name, symbol),
                *ALERT_ROW_COLOR,
            ),
            'P' => Line::styled(format!("{}) {}", index, self.name), *MARKER_COLOR),
            _ => Line::styled(format!("{}) {}", index, self.name), *TEXT_COLOR),
        };

        ListItem::new(line)
    }
}

#[derive(Debug, Default, Clone, Getters, MutGetters, Setters)]
struct RegionsList {
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    items: Vec<ListItem<'static>>,
    #[getset(get = "pub", get_mut = "pub")]
    state: ListState,
    #[getset(get = "pub", get_mut = "pub")]
    last_selected: Option<usize>,
}

impl RegionsList {
    pub fn new(regions: &[Region], alerts_statuses: &[char]) -> Self {
        let items: Vec<ListItem> = regions
            .iter()
            .enumerate()
            .map(|(i, r)| r.to_list_item(i as i8, alerts_statuses[i]))
            .collect();
        let state = ListState::default();
        let last_selected = None;
        Self {
            items,
            state,
            last_selected,
        }
    }

    #[tracing::instrument(skip(self))]
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => self.last_selected.unwrap_or(0),
        };
        self.state.select(Some(i));
        // info!("List->next, selected region: {:?}", i);
    }

    #[tracing::instrument(skip(self))]
    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => self.last_selected.unwrap_or(0),
        };
        self.state.select(Some(i));
        // info!("List->previous, selected region: {:?}", i);
    }

    pub fn unselect(&mut self) {
        let offset = self.state.offset();
        self.last_selected = self.state.selected();
        self.state.select(None);
        *self.state.offset_mut() = offset;
    }

    pub fn go_top(&mut self) {
        self.state.select(Some(0));
    }

    pub fn go_bottom(&mut self) {
        self.state.select(Some(self.items.len() - 1));
    }
}

#[derive(Debug, Default, Getters, Setters)]
pub struct Ukraine {
    borders: String,
    #[getset(get = "pub")]
    regions: RegionArrayVec,
    #[getset(get = "pub", set = "pub")]
    size: Rect,
    #[getset(get = "pub", set = "pub")]
    list: RegionsList,
}

impl Ukraine {
    pub fn new(
        borders: String,
        regions: RegionArrayVec,
        alerts_string: AlertsResponseString,
    ) -> Self {
        let bbox = Rect::default();
        let alerts_statuses: Vec<char> = alerts_string.chars().collect::<Vec<char>>();
        let list = RegionsList::new(regions.as_slice(), alerts_statuses.as_slice());

        Self {
            borders,
            regions,
            size: bbox,
            list,
        }
    }

    delegate! {
        to self.list {
            #[call(items)]
            pub fn get_list_items(&mut self) -> &Vec<ListItem<'static>>;
            #[call(set_items)]
            pub fn set_list_items(&mut self, items: Vec<ListItem<'static>>);
            pub fn next(&mut self);
            pub fn previous(&mut self);
            pub fn unselect(&mut self);
            pub fn go_top(&mut self);
            pub fn go_bottom(&mut self);
        }

        to self.list.items {
            #[call(len)]
            pub fn list_size(&self) -> usize;
            // pub fn items(&self) -> &Vec<Region>;
        }
    }

    pub fn borders(&self) -> Polygon {
        use std::str::FromStr;
        use wkt::Wkt;
        let geom: Polygon = Wkt::from_str(&self.borders).unwrap().try_into().unwrap();
        geom
    }

    pub fn list_state(&self) -> &ListState {
        self.list.state()
    }

    /// update list items with alerts and change item status
    pub fn set_alerts(&mut self, alerts: Vec<Alert>) {
        info!("Ukraine->set_alerts: {:?}", alerts);
        let mut regions = ArrayVec::<Region, 27>::new();
        self.regions.iter_mut().for_each(|item| {
            if let Some(alert) = alerts
                .iter()
                .find(|alert| alert.location_oblast_uid.unwrap() == item.id as i32)
            {
                if Some(alert).is_some() {
                    item.status = Some("A".to_string());
                }
            } else {
                item.status = None;
            }
            regions.push(item.clone());
        });

        self.regions = regions
    }
}
