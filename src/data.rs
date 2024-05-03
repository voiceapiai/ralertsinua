use crate::{
    fs::{self},
    utils::*,
};
use async_trait::async_trait;
use color_eyre::eyre::{Context, Result};
use core::str;
use getset::Getters;
use ralertsinua_http::*;
use ralertsinua_models::*;
#[allow(unused)]
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
#[allow(unused)]
use tracing::error;

#[allow(unused)]
const FILE_PATH_CSV: &str = ".data/ukraine.csv";
#[allow(unused)]
const FILE_PATH_WKT: &str = ".data/ukraine.wkt";
const DB_NAME: &str = "ukraine.sqlite";
// const DB_PATH: &'static str = ".data/ukraine.sqlite";
const QUERY_CREATE_REGIONS_TABLE: &str = include_str!("../.data/create_regions_table.sql");
const QUERY_SELECT_REGIONS: &str = "SELECT * FROM regions ORDER BY id";
const QUERY_SELECT_REGION_GEO: &str = "SELECT geo FROM geo WHERE osm_id = $1";

/// The `DataRepository` trait provides methods for interacting with a SQLite database and fetching data related to Ukraine.
#[async_trait]
pub trait DataRepository: Send + Sync + core::fmt::Debug {
    async fn fetch_regions(&self) -> Result<Box<[Region]>>;

    async fn fetch_region_geo(&self, osm_id: i64) -> Result<String>;

    async fn fetch_borders(&self) -> Result<String>;

    async fn fetch_alerts(&self) -> Result<Vec<Alert>>;

    async fn fetch_alerts_string(&self) -> Result<Box<String>>;
}

#[tracing::instrument(level = "trace")]
pub async fn db_pool() -> Result<SqlitePool> {
    let path = if cfg!(debug_assertions) {
        get_local_data_dir()
    } else {
        get_config_dir()
    };
    let db_path = path.join(DB_NAME);
    let conn: SqliteConnectOptions = SqliteConnectOptions::new()
        .filename(path.join(DB_NAME))
        // .pragma(key, value)
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(conn).await.wrap_err(format!(
        "Error connecting to the database with path {}",
        db_path.to_str().unwrap()
    ))?;
    // Create the tables together with the pool
    DataRepositoryInstance::create_tables(&pool).await?;
    DataRepositoryInstance::insert_regions_geo(&pool).await?;

    Ok(pool)
}

#[derive(Debug, Getters)]
pub struct DataRepositoryInstance {
    /// The HTTP client
    #[getset(get = "pub")]
    client: AlertsInUaClient,
    /// The database pool.
    #[getset(get = "pub")]
    pool: SqlitePool,
}

impl DataRepositoryInstance {
    pub fn new(pool: SqlitePool, client: AlertsInUaClient) -> Self {
        Self { client, pool }
    }

    async fn create_tables(pool: &SqlitePool) -> Result<()> {
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
    }
}

#[async_trait]
impl DataRepository for DataRepositoryInstance {
    async fn fetch_regions(&self) -> Result<Box<[Region]>> {
        let regions: Vec<Region> = sqlx::query_as(QUERY_SELECT_REGIONS)
            .fetch_all(self.pool())
            .await
            .wrap_err("Error querying regions from the database: {}")?;

        Ok(regions.into_boxed_slice())
    }

    async fn fetch_region_geo(&self, osm_id: i64) -> Result<String> {
        let geo_string: String = sqlx::query_scalar(QUERY_SELECT_REGION_GEO)
            .bind(osm_id)
            .fetch_one(self.pool())
            .await
            .wrap_err("Error querying region's geo from the database: {}")?;

        Ok(geo_string)
    }

    async fn fetch_borders(&self) -> Result<String> {
        let borders = fs::read_file_into_string(FILE_PATH_WKT)?;
        Ok(borders)
    }

    async fn fetch_alerts(&self) -> Result<Vec<Alert>> {
        let response: AlertsResponseAll = self
            .client
            .get_active_alerts()
            .await
            .wrap_err("Error fetching alerts from API: {}")?;

        // info!("Fetched {} alerts", response.alerts.len());
        Ok(response.alerts)
    }

    /// Fetches active air raid alerts **as string** from alerts.in.ua
    ///
    /// Example response: `"ANNNANNNNNNNANNNNNNNNNNNNNN"`
    async fn fetch_alerts_string(&self) -> Result<Box<String>> {
        let response: String = self
            .client()
            .get_air_raid_alert_statuses_by_region()
            .await
            .wrap_err("Error fetching alerts from API: {}")?;
        let text = response.trim_matches('"');
        // info!("Fetched alerts as string: {}, length: {}", text, text.len());
        let text = Box::new(text.to_string());

        // Insert the response into the statuses table
        sqlx::query("INSERT INTO statuses (status) VALUES (?)")
            .bind(&*text)
            .execute(self.pool())
            .await
            .wrap_err("Error inserting status into the database: {}")?;

        Ok(text)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::config::*;
    use mockito::Server as MockServer;
    use sqlx::Pool;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_fetch_alerts_string() -> Result<()> {
        let mut server = MockServer::new_async().await;
        let mut config = Config::init().unwrap();
        config.set_base_url(server.url());
        let config: Arc<dyn ConfigService> = Arc::new(config);
        let mock = server
            .mock(
                "GET",
                mockito::Matcher::Any, /* API_ALERTS_ACTIVE_BY_REGION_STRING */
            )
            .with_body(r#""ANNAANNANNNPANANANNNNAANNNN""#)
            .create_async()
            .await;
        let client = AlertsInUaClient::new(config.base_url(), config.token());
        let pool = Pool::connect("sqlite::memory:").await?;
        DataRepositoryInstance::create_tables(&pool).await?;
        let data_repository = DataRepositoryInstance::new(pool, client);

        let result = data_repository.fetch_alerts_string().await?;

        mock.assert();
        assert_eq!(&*result, "ANNAANNANNNPANANANNNNAANNNN");

        Ok(())
    }

    /// JSON string example to match later
    #[allow(unused)]
    const DEMO_ALERTS_RESPONSE: &str = r#"
        {"alerts":[{"id":8757,"location_title":"Луганська область","location_type":"oblast","started_at":"2022-04-04T16:45:39.000Z","finished_at":null,"updated_at":"2023-10-29T18:22:37.357Z","alert_type":"air_raid","location_oblast":"Луганська область","location_uid":"16","notes":null,"country":null,"calculated":null,"location_oblast_uid":16},{"id":28288,"location_title":"Автономна Республіка Крим","location_type":"oblast","started_at":"2022-12-10T22:22:00.000Z","finished_at":null,"updated_at":"2023-10-29T16:56:12.340Z","alert_type":"air_raid","location_oblast":"Автономна Республіка Крим","location_uid":"29","notes":"Згідно інформації з Офіційних карт тривог","country":null,"calculated":null,"location_oblast_uid":29},{"id":71710,"location_title":"Мирівська територіальна громада","location_type":"hromada","started_at":"2024-04-18T05:43:26.000Z","finished_at":null,"updated_at":"..."}]}
    "#;
}
