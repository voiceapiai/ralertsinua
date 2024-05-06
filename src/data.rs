use async_trait::async_trait;
use color_eyre::eyre::{Result, WrapErr};
use delegate::delegate;
// use getset::Getters;
use ralertsinua_geo::AlertsInUaGeo;
use ralertsinua_http::*;
use ralertsinua_models::*;
#[allow(unused)]
use tracing::error;

pub use std::sync::{Arc, RwLock};

/// The `DataRepository` trait provides methods for interacting with a SQLite database and fetching data related to Ukraine.
#[async_trait]
pub trait AlertsInUaFacade: Send + Sync + core::fmt::Debug {
    fn borders(&self) -> &str;
    fn regions(&self) -> &'static [Region; 27];

    // async fn fetch_region_geo(&self, osm_id: i32) -> Result<String>;

    async fn fetch_alerts(&self) -> Result<Alerts>;

    async fn fetch_alerts_string(&self) -> Result<String>;
}

#[derive(Debug)]
pub struct AlertsInUaContainer {
    // #[getset(get = "pub")]
    api_client: AlertsInUaClient,
    // #[getset(get = "pub")]
    geo_client: AlertsInUaGeo,
}

impl AlertsInUaContainer {
    pub fn new(api_client: AlertsInUaClient, geo_client: AlertsInUaGeo) -> Self {
        Self {
            api_client,
            geo_client,
        }
    }

    /* async fn create_tables(pool: &SqlitePool) -> Result<()> {
        sqlx::query(QUERY_CREATE_REGIONS_TABLE)
            .execute(pool)
            .await
            .wrap_err("Error creating sqlite tables: {}")?;
        Ok(())
    }

    async fn insert_regions_geo(pool: &SqlitePool) -> Result<()> {
        let count: i8 = sqlx::query_scalar("SELECT COUNT(*) FROM geo")
            .fetch_one(pool)
            .await
            .wrap_err("Error querying geo table: {}")?;

        if count > 0 {
            return Ok(());
        }
        let data = fs::read_csv_file_into::<RegionGeo>(FILE_PATH_CSV)?;

        for region in data.iter() {
            sqlx::query("INSERT INTO geo (osm_id,geo) VALUES (?, ?)")
                .bind(region.osm_id)
                .bind(region.geo.as_str())
                .execute(pool)
                .await
                .wrap_err("Error inserting regions into the database: {}")?;
        }

        Ok(())
    } */
}

#[async_trait]
impl AlertsInUaFacade for AlertsInUaContainer {
    delegate! {
        to self.geo_client {
            fn borders(&self) -> &str;
            fn regions(&self) -> &'static [Region; 27];
        }
    }

    /* async fn fetch_region_geo(&self, osm_id: i32) -> Result<String> {
         let geo_string: String = sqlx::query_scalar(QUERY_SELECT_REGION_GEO)
            .bind(osm_id)
            .fetch_one(self.pool())
            .await
            .wrap_err("Error querying region's geo from the database: {}")?;

        Ok(geo_string)
    } */

    async fn fetch_alerts(&self) -> Result<Alerts> {
        let response: Alerts = self
            .api_client
            .get_active_alerts()
            .await
            .wrap_err("Error fetching alerts from API: {}")?;

        // info!("Fetched {} alerts", response.alerts.len());
        Ok(response)
    }

    /// Fetches active air raid alerts **as string** from alerts.in.ua
    ///
    /// Example response: `"ANNNANNNNNNNANNNNNNNNNNNNNN"`
    async fn fetch_alerts_string(&self) -> Result<String> {
        let response: String = self
            .api_client
            .get_air_raid_alert_statuses_by_region()
            .await
            .wrap_err("Error fetching alerts from API: {}")?;
        let text = response.trim_matches('"').to_string();

        Ok(text)
    }
}
