/// This module contains the implementation of the `DataRepository` struct and the `MapRepository` trait.
/// The `DataRepository` struct provides methods for interacting with a SQLite database and fetching data related to Ukraine.
/// The `MapRepository` trait defines the `get_data` method, which returns a future that resolves to a `Result` containing the data for Ukraine.
use crate::{
    alerts::*,
    ukraine::{Region, RegionArrayVec, Ukraine},
};
use arrayvec::ArrayString;
use color_eyre::eyre::{Context, Error, Result};
use core::str;
use getset::Getters;
#[allow(unused)]
use reqwest::Client;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use std::{fs::File, future::Future, io::Read, result::Result::Ok, sync::Arc, vec};
use strum::Display;
use tracing::{error, info};

#[allow(unused)]
const FILE_PATH_CSV: &'static str = ".data/ukraine.csv";
#[allow(unused)]
const FILE_PATH_WKT: &'static str = ".data/ukraine.wkt";
const DB_PATH: &'static str = ".data/ukraine.sqlite";
const QUERY_CREATE_REGIONS_TABLE: &'static str = include_str!("../.data/create_regions_table.sql");
const QUERY_SELECT_REGIONS: &'static str = "SELECT * FROM regions ORDER BY id";

/// The `API` module contains constants related to the alerts.in.ua API.
#[allow(non_snake_case)]
pub mod API {
    use lazy_static::lazy_static;
    use crate::config::CONFIG;

    lazy_static! {
        pub static ref API_TOKEN: String = CONFIG.read().unwrap().get::<String>("settings.token").unwrap();
        pub static ref API_BASE_URL: String = "https://api.alerts.in.ua".to_string();
        pub static ref ALERTS_ACTIVE: String = format!(
            "{}/v1/alerts/active.json?token={}",
            API_BASE_URL.as_str(),
            API_TOKEN.as_str()
        );
        pub static ref ALERTS_ACTIVE_BY_REGION_STRING: String = format!(
            "{}/v1/iot/active_air_raid_alerts_by_oblast.json?token={}",
            API_BASE_URL.as_str(),
            API_TOKEN.as_str()
        );
    }
}

#[tracing::instrument(level = "trace")]
pub async fn db_pool() -> SqlitePool {
    let conn: SqliteConnectOptions = SqliteConnectOptions::new()
        .filename(DB_PATH)
        .create_if_missing(true);

    let pool = match SqlitePool::connect_with(conn).await {
        Ok(pool) => {
            info!("SQLite database {} connected successfully", DB_PATH);
            pool
        }
        Err(e) => {
            error!("Error connecting to sqlite database: {}", e);
            panic!("Error connecting to sqlite database: {}", e);
        }
    };
    match sqlx::query(QUERY_CREATE_REGIONS_TABLE).execute(&pool).await {
        Ok(_) => {
            info!("SQLite table created successfully");
        }
        Err(e) => {
            error!("Error creating sqlite table: {}", e);
            drop(e);
        }
    }
    // Return the pool
    pool
}

#[derive(Debug, Getters)]
pub struct DataRepository {
    /// The HTTP client
    #[getset(get = "pub")]
    client: Client,
    /// The database pool.
    #[getset(get = "pub")]
    pool: SqlitePool,
}

impl DataRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            client: Client::new(),
            pool,
        }
    }

    #[tracing::instrument(level = "info")]
    fn open_file(file_path: &str) -> Result<File> {
        return File::open(file_path).wrap_err("Error opening file, {}");
    }

    #[allow(unused)]
    fn read_csv_file(file_path: &str) -> Result<Vec<Region>> {
        use csv::ReaderBuilder;
        let mut records = vec![];
        let file = Self::open_file(file_path)?;

        let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

        for result in rdr.deserialize() {
            let record: Region = match result {
                Ok(r) => r,
                Err(e) => {
                    panic!("Error reading CSV file: {}", e);
                }
            };
            records.push(record);
        }

        Ok(records)
    }

    fn read_wkt_file(file_path: &str) -> Result<String> {
        let mut file = Self::open_file(file_path)?;
        let mut wkt_string = String::new();
        file.read_to_string(&mut wkt_string)?;

        Ok(wkt_string)
    }

    /*
    async fn insert_regions(&self, data: &[Region]) -> Result<()> {
        for region in data.iter() {
            sqlx::query(QUERY_INSERT_REGIONS)
                .bind(region.id)
                .bind(region.a_id)
                .bind(region.geo.as_str())
                .bind(region.name.as_str())
                .bind(region.name_en.as_str())
                .execute(self.pool())
                .await
                .with_context(|| "Error inserting regions into the database: {}")?;
        }

        Ok(())
    } */

    pub async fn fetch_regions(&self) -> Result<RegionArrayVec> {
        use arrayvec::ArrayVec;
        let regions: Vec<Region> = sqlx::query_as(QUERY_SELECT_REGIONS)
            .fetch_all(self.pool())
            .await
            .wrap_err("Error querying regions from the database: {}")?;

        Ok(ArrayVec::<Region, 27>::from_iter(regions))
    }

    pub async fn fetch_borders(&self) -> Result<String> {
        let borders = Self::read_wkt_file(FILE_PATH_WKT)?;
        Ok(borders)
    }

    pub async fn fetch_alerts(&self) -> Result<Vec<Alert>> {
        let response: AlertsResponseAll = self
            .client
            .get(API::ALERTS_ACTIVE.as_str())
            .send()
            .await
            .wrap_err("Error fetching alerts from API: {}")?
            .json::<AlertsResponseAll>()
            .await
            .wrap_err("Error parsing alerts response: {}")?;

        info!("Fetched {} alerts", response.alerts.len());
        Ok(response.alerts)
    }

    /// Fetches active air raid alerts as string from alerts.in.ua
    ///
    /// Response: `"ANNNANNNNNNNANNNNNNNNNNNNNN"`
    pub async fn fetch_alerts_string(&self) -> Result<AlertsResponseString> {
        let url = API::ALERTS_ACTIVE_BY_REGION_STRING.as_str();
        let response = self
            .client()
            .get(url)
            .send()
            .await
            .wrap_err("Error fetching alerts from API: {}")?;
        let content: String = response
            .text()
            .await
            .wrap_err("Error parsing alerts response: {}")?;
        let text = content.trim_matches('"');
        info!("Fetched alerts as string: {}, length: {}", text, text.len());
        let mut a_string = ArrayString::<27>::new();
        a_string.push_str(&text);

        Ok(a_string)
    }
}
