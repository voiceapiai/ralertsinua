use crate::{error::*, ukraine::*, DataRepository};
use async_trait::async_trait;
use color_eyre::eyre::{Context, Error, Result};
use std::sync::Arc;

#[async_trait]
pub trait GeoService: Sync + Send {
    async fn fetch_borders(&self) -> Result<String>;
    async fn fetch_region_geo(&self, osm_id: i64) -> Result<String>;
}

#[derive(Debug, Clone)]
pub struct GeoServiceImpl {
    pub repository: Arc<dyn DataRepository>,
}

impl GeoServiceImpl {
    pub fn new(repository: Arc<dyn DataRepository>) -> Self {
        GeoServiceImpl { repository }
    }
}

#[async_trait]
impl GeoService for GeoServiceImpl {
    async fn fetch_borders(&self) -> Result<String> {
        Ok(self.repository.fetch_borders().await?)
    }

    async fn fetch_region_geo(&self, osm_id: i64) -> Result<String> {
        Ok(self.repository.fetch_region_geo(osm_id).await?)
    }
}
