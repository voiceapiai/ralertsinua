use async_trait::async_trait;
use color_eyre::eyre::{Result, WrapErr};
use delegate::delegate;
use getset::Getters;
// use getset::Getters;
use ralertsinua_geo::AlertsInUaGeo;
use ralertsinua_http::*;
use ralertsinua_models::*;
use tracing::info;

pub use std::sync::{Arc, RwLock};

/// The `DataRepository` trait provides methods for interacting with a SQLite database and fetching data related to Ukraine.
#[async_trait]
pub trait AlertsInUaFacade: Send + Sync + core::fmt::Debug {
    fn borders(&self) -> &str;
    fn regions(&self) -> &'static [Region; 27];

    fn oblast_statuses(&self) -> Vec<AirRaidAlertOblastStatus>; // std::slice::Iter<AirRaidAlertOblastStatus>;

    // async fn fetch_region_geo(&self, osm_id: i32) -> Result<String>;

    async fn fetch_active_alerts(&self) -> Result<()>;

    async fn fetch_air_raid_alert_statuses_by_region(&self) -> Result<()>;
}

#[derive(Debug, Getters)]
pub struct AlertsInUaContainer {
    // #[getset(get = "pub")]
    api_client: AlertsInUaClient,
    // #[getset(get = "pub")]
    geo_client: AlertsInUaGeo,
    // #[getset(get = "pub")]
    oblast_statuses: RwLock<AirRaidAlertOblastStatuses>,
}

impl AlertsInUaContainer {
    pub fn new(api_client: AlertsInUaClient, geo_client: AlertsInUaGeo) -> Self {
        Self {
            api_client,
            geo_client,
            oblast_statuses: RwLock::new(AirRaidAlertOblastStatuses::default()),
        }
    }
}

#[async_trait]
impl AlertsInUaFacade for AlertsInUaContainer {
    delegate! {
        to self.geo_client {
            fn borders(&self) -> &str;
            fn regions(&self) -> &'static [Region; 27];
        }
    }

    fn oblast_statuses(&self) -> Vec<AirRaidAlertOblastStatus> {
        self.oblast_statuses
            .read()
            .unwrap()
            .iter()
            .cloned()
            .collect()
    }

    /// Fetches active alerts from alerts.in.ua
    ///
    async fn fetch_active_alerts(&self) -> Result<()> {
        let response: Alerts = self
            .api_client
            .get_active_alerts()
            .await
            .wrap_err("Error fetching alerts from API: {}")?;

        info!("fetch_alerts: total {} alerts", response.len());
        response.iter().for_each(|alert| {
            info!("fetch_alerts:alert {:?}", alert);
        });

        Ok(())
    }

    /// Fetches active air raid alerts **as string** from alerts.in.ua
    ///
    async fn fetch_air_raid_alert_statuses_by_region(&self) -> Result<()> {
        let result = self
            .api_client
            .get_air_raid_alert_statuses_by_region()
            .await
            .wrap_err("Error fetching alerts from API: {}")?;

        *self.oblast_statuses.write().unwrap() = result;

        Ok(())
    }
}
