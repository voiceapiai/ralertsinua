use crate::{alerts::*, constants::*};
#[allow(unused)]
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
pub use std::sync::{Arc, RwLock};
use tracing::info;

// use geo::algorithm::bounding_rect::BoundingRect;
// use geo::algorithm::simplify_vw::SimplifyVw;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Region {
    pub id: i8,
    pub a_id: i8,
    pub osm_id: i64,
    // #[sqlx(rename = "geo", default)]
    // pub geo: String,
    pub name: String,
    pub name_en: String,
}
#[derive(Debug, Default, Deserialize, sqlx::FromRow)]
pub struct RegionGeo {
    pub osm_id: i64,
    pub a_id: i8,
    pub geo: String,
}
pub type RegionArrayVec = ArrayVec<Region, 27>;
pub type RegionListVec<'a> = ArrayVec<ListItem<'a>, 27>;

#[derive(Debug, Default, Getters, Setters)]
pub struct Ukraine {
    #[getset(get = "pub")]
    regions: RegionArrayVec,
}

impl Ukraine {
    pub fn new_arc() -> Arc<RwLock<Ukraine>> {
        Arc::new(RwLock::new(Ukraine::default()))
    }

    pub fn new(regions: RegionArrayVec) -> Self {
        Self { regions }
    }
}
