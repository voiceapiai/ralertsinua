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
pub trait RegionsState {}
pub type RegionArrayVec = ArrayVec<Region, 27>;
pub type RegionListVec<'a> = ArrayVec<ListItem<'a>, 27>;

#[derive(Debug, Default, Getters, Setters)]
pub struct Ukraine {
    /// ArrayVec of regions
    #[getset(get = "pub")]
    regions: RegionArrayVec,
    /// Alerts by region string as state
    #[getset(get = "pub")]
    alerts_state: Option<Box<dyn AlertsByRegionState>>,
}

impl Ukraine {
    pub fn new(regions: RegionArrayVec) -> Self {
        let alerts_state = Some(Box::new(AlertsByRegion::default()) as Box<dyn AlertsByRegionState>);
        Self {
            regions,
            alerts_state,
        }
    }

    /* delegate! {
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
    } */

    #[deprecated = "TODO"]
    pub fn set_alerts_full(&mut self, alerts: Vec<Alert>) {
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

    pub fn set_alerts(&mut self, alerts_as: AlertsResponseString) {
        self.alerts_state = Some(self.alerts_state.take().unwrap().set_alerts(alerts_as));
        info!("Ukraine->set_alerts: {:?}", self.alerts_state);
    }

    pub fn get_alerts<'a>(&'a self) -> &'a str {
        self.alerts_state.as_ref().unwrap().get_alerts()
    }
}
