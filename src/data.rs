/// This module contains the implementation of the `DataRepository` struct and the `MapRepository` trait.
/// The `DataRepository` struct provides methods for interacting with a SQLite database and fetching data related to Ukraine.
/// The `MapRepository` trait defines the `get_data` method, which returns a future that resolves to a `Result` containing the data for Ukraine.
use crate::{alerts::*, api::*, ukraine::*, utils::*};
use arrayvec::ArrayString;
use async_trait::async_trait;
use color_eyre::eyre::{Context, Error, Result};
use core::str;
use getset::Getters;
use serde::Deserialize;
#[allow(unused)]
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use std::{fs::File, future::Future, io::Read, result::Result::Ok, sync::Arc, vec};
use strum::Display;
use tracing::{error, info};

#[allow(unused)]
const FILE_PATH_CSV: &'static str = ".data/ukraine.csv";
#[allow(unused)]
const FILE_PATH_WKT: &'static str = ".data/ukraine.wkt";
const DB_NAME: &'static str = "ukraine.sqlite";
// const DB_PATH: &'static str = ".data/ukraine.sqlite";
const QUERY_CREATE_REGIONS_TABLE: &'static str = include_str!("../.data/create_regions_table.sql");
const QUERY_SELECT_REGIONS: &'static str = "SELECT * FROM regions ORDER BY id";
const QUERY_SELECT_REGION_GEO: &'static str = "SELECT geo FROM geo WHERE osm_id = $1";

#[async_trait]
pub trait DataRepository: Send + Sync + core::fmt::Debug {
    async fn fetch_regions(&self) -> Result<RegionArrayVec>;

    async fn fetch_region_geo(&self, osm_id: i64) -> Result<String>;

    async fn fetch_borders(&self) -> Result<String>;

    async fn fetch_alerts(&self) -> Result<Vec<Alert>>;

    async fn fetch_alerts_string(&self) -> Result<AlertsResponseString>;
}

#[tracing::instrument(level = "trace")]
pub async fn db_pool() -> Result<SqlitePool> {
    let path = if cfg!(debug_assertions) {
        get_local_data_dir()
    } else {
        get_config_dir()
    };
    let conn: SqliteConnectOptions = SqliteConnectOptions::new()
        .filename(path.join(DB_NAME))
        // .pragma(key, value)
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(conn)
        .await
        .wrap_err("Error connecting to the database: {}")?;
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
        let data = Self::read_csv_file_into::<RegionGeo>(FILE_PATH_CSV)?;

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

    #[tracing::instrument]
    fn open_file(file_path: &str) -> Result<File> {
        return File::open(file_path).wrap_err("Error opening file, {}");
    }

    #[tracing::instrument]
    fn read_csv_file_into<R>(file_path: &str) -> Result<Vec<R>>
    where
        R: for<'de> Deserialize<'de> + Default,
    {
        use csv::ReaderBuilder;
        let file = Self::open_file(file_path)?;
        let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);
        let data = rdr
            .deserialize::<R>()
            .map(|r| {
                let rg: R = match r {
                    Ok(rg) => rg,
                    Err(e) => {
                        error!("Error deserializing csv row: {}", e);
                        return R::default();
                    }
                };
                rg
            })
            .collect::<Vec<R>>();

        Ok(data)
    }

    fn read_wkt_file(file_path: &str) -> Result<String> {
        let mut file = Self::open_file(file_path)?;
        let mut wkt_string = String::new();
        file.read_to_string(&mut wkt_string)?;

        Ok(wkt_string)
    }
}

#[async_trait]
impl DataRepository for DataRepositoryInstance {
    async fn fetch_regions(&self) -> Result<RegionArrayVec> {
        use arrayvec::ArrayVec;
        let regions: Vec<Region> = sqlx::query_as(QUERY_SELECT_REGIONS)
            .fetch_all(self.pool())
            .await
            .wrap_err("Error querying regions from the database: {}")?;

        Ok(ArrayVec::<Region, 27>::from_iter(regions))
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
        let borders = Self::read_wkt_file(FILE_PATH_WKT)?;
        Ok(borders)
    }

    async fn fetch_alerts(&self) -> Result<Vec<Alert>> {
        let response: AlertsResponseAll = self
            .client
            .get(API_ALERTS_ACTIVE, None)
            .await
            .wrap_err("Error fetching alerts from API: {}")?;

        // info!("Fetched {} alerts", response.alerts.len());
        Ok(response.alerts)
    }

    /// Fetches active air raid alerts **as string** from alerts.in.ua
    ///
    /// Example response: `"ANNNANNNNNNNANNNNNNNNNNNNNN"`
    async fn fetch_alerts_string(&self) -> Result<AlertsResponseString> {
        let response: String = self
            .client()
            .get(API_ALERTS_ACTIVE_BY_REGION_STRING, None)
            .await
            .wrap_err("Error fetching alerts from API: {}")?;
        let text = response.trim_matches('"');
        // info!("Fetched alerts as string: {}, length: {}", text, text.len());
        let res = Box::new(text.to_string());
        let mut a_string = ArrayString::<27>::new();
        a_string.push_str(&text);

        // Insert the response into the statuses table
        sqlx::query("INSERT INTO statuses (status) VALUES (?)")
            .bind(&text)
            .execute(self.pool())
            .await
            .wrap_err("Error inserting status into the database: {}")?;

        Ok(a_string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server as MockServer;
    use reqwest::Client;
    use sqlx::{Connection, Pool, SqliteConnection};
    use tokio::runtime::Runtime;

    #[tokio::test]
    async fn test_fetch_alerts_string() -> Result<()> {
        std::env::set_var("ALERTSINUA_TOKEN", "TEST_TOKEN");
        let mut server = MockServer::new_async().await;
        let mock = server
            .mock(
                "GET",
                mockito::Matcher::Any, /* API_ALERTS_ACTIVE_BY_REGION_STRING */
            )
            .with_body(r#""ANNAANNANNNPANANANNNNAANNNN""#)
            .create_async()
            .await;
        let mut client = AlertsInUaClient::default();
        client.set_base_url(server.url());
        let pool = Pool::connect("sqlite::memory:").await?;
        let ready = DataRepositoryInstance::create_tables(&pool).await?;
        let data_repository = DataRepositoryInstance::new(pool, client);

        let result = data_repository.fetch_alerts_string().await?;

        mock.assert();
        assert_eq!(result.len(), 27);
        assert_eq!(&result, "ANNAANNANNNPANANANNNNAANNNN");

        Ok(())
    }
}
