#![allow(clippy::borrow_deref_ref)]
// use async_trait::async_trait;
// use color_eyre::eyre::Result;
use crate::constants::*;
use getset::Getters;
use ralertsinua_models::Region;

pub type WktString = String;

/* #[async_trait]
pub trait AlertsInUaGeoService: Sync + Send {
    fn borders(&self) -> Result<WktString>;
    fn regions(&self) -> Result<&'static [Region; 27]>;
    // async fn fetch_region_geo(&self, osm_id: i64) -> Result<String>;
} */

#[derive(Debug, Clone, Getters)]
pub struct AlertsInUaGeo {
    #[getset(get = "pub")]
    borders: &'static str,
    #[getset(get = "pub")]
    regions: &'static [Region; 27],
}

// #[async_trait]
impl AlertsInUaGeo {
    #[inline]
    pub fn new() -> Self {
        Self {
            borders: &*UKRAINE_BORDERS_POYGON_WKT,
            regions: &*UKRAINE_REGIONS,
        }
    }
}

impl Default for AlertsInUaGeo {
    fn default() -> Self {
        Self::new()
    }
}
