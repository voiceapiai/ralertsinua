use crate::{alerts::*, constants::*};
#[allow(unused)]
use anyhow::*;
use arrayvec::ArrayVec;
use derive_deref::Deref;
use geo::{Coord, Polygon};
use getset::{Getters, MutGetters, Setters};
use ratatui::{
    layout::Rect,
    prelude::*,
    widgets::{ListItem, ListState},
};
use serde::*;
pub use std::sync::{Arc, Mutex};
use tracing::info;

// pub type UkraineArc = Arc<Mutex<Ukraine>>;
#[derive(Debug, Deref, Default)]
pub struct UkraineArc(Arc<Mutex<Ukraine>>);

// use geo::algorithm::bounding_rect::BoundingRect;
// use geo::algorithm::simplify_vw::SimplifyVw;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Region {
    pub id: i8,
    pub a_id: i8,
    pub osm_id: i64,
    pub geo: String,
    pub name: String,
    pub name_en: String,
    // #[sqlx(rename = "status", default)]
    // pub status: Option<String>,
}
pub trait RegionsState {}
pub type RegionArrayVec = ArrayVec<Region, 27>;
pub type RegionListVec<'a> = ArrayVec<ListItem<'a>, 27>;

#[derive(Debug, Default, Getters, Setters)]
pub struct Ukraine {
    /// ArrayVec of regions
    #[getset(get = "pub", set = "pub")]
    regions: RegionArrayVec,
    /// Alerts by region string as state
    #[getset(get = "pub")]
    alerts_state: Option<Box<dyn AlertsByRegionState>>,
}

impl Ukraine {
    pub fn new(regions: RegionArrayVec) -> Self {
        let alerts_state =
            Some(Box::new(AlertsByRegion::default()) as Box<dyn AlertsByRegionState>);
        Self {
            regions,
            alerts_state,
        }
    }

    pub fn set_alerts(&mut self, alerts_as: AlertsResponseString) {
        let alerts_state =
            Some(Box::new(AlertsByRegion::new(alerts_as)) as Box<dyn AlertsByRegionState>);
        self.alerts_state = alerts_state;
        info!("Ukraine->set_alerts: {:?}", self.alerts_state);
    }

    pub fn get_alerts<'a>(&'a self) -> &'a str {
        let alerts_state = if let Some(state) = self.alerts_state.as_ref() {
            state.get_alerts()
        } else {
            "NNNNNNNNNNNNNNNNNNNNNNNNNNN"
        };
        alerts_state
    }
}
