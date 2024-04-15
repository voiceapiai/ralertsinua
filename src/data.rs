/// This module contains the implementation of the `DataRepository` struct and the `MapRepository` trait.
/// The `DataRepository` struct provides methods for interacting with a SQLite database and fetching data related to Ukraine.
/// The `MapRepository` trait defines the `get_data` method, which returns a future that resolves to a `Result` containing the data for Ukraine.
// use geozero::{csv::*, error::*, wkt::*};
use crate::ukraine::*;
use anyhow::*;
use geo::{Coord, CoordsIter, Geometry};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use std::{fs::File, future::Future, io::Read, result::Result::Ok, sync::Arc, vec};
use tracing::{error, info};

const FILE_PATH_CSV: &'static str = "data/ukraine.csv";
const FILE_PATH_WKT: &'static str = "data/ukraine.wkt";
const DB_PATH: &'static str = "data/ukraine.sqlite";
const QUERY_CREATE_TABLE: &'static str = "
CREATE TABLE IF NOT EXISTS regions (
id INTEGER PRIMARY KEY,
geo TEXT NOT NULL,
name TEXT NOT NULL,
name_en TEXT NOT NULL
)";
const QUERY_INSERT_REGIONS: &'static str = "
INSERT INTO regions (id, geo, name, name_en) VALUES (?, ?, ?, ?)
ON CONFLICT(id) DO UPDATE SET
geo = excluded.geo,
name = excluded.name,
name_en = excluded.name_en";
const QUERY_SELECT_REGIONS: &'static str = "SELECT * FROM regions";

#[tracing::instrument(level = "trace")]
pub async fn db_pool() -> Arc<SqlitePool> {
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
    match sqlx::query(QUERY_CREATE_TABLE).execute(&pool).await {
        Ok(_) => {
            info!("SQLite table created successfully");
        }
        Err(e) => {
            error!("Error creating sqlite table: {}", e);
            drop(e);
        }
    }
    // Return the pool
    Arc::new(pool)
}

#[derive(Debug)]
pub struct DataRepository {
    /// The database pool.
    pool: Arc<SqlitePool>,
}

pub trait MapRepository {
    fn get_data(&mut self) -> impl Future<Output = Result<Ukraine>> + Send;
}

impl DataRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    fn pool(&self) -> &SqlitePool {
        self.pool.as_ref()
    }

    #[tracing::instrument(level = "info")]
    fn open_file(file_path: &str) -> Result<File> {
        return File::open(file_path)
            .with_context(|| format!("Error opening file '{}':", file_path));
    }

    #[tracing::instrument]
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

    #[tracing::instrument]
    fn read_wkt_file(file_path: &str) -> Result<Vec<Coord>> {
        use std::str::FromStr;
        use wkt::Wkt;
        let mut file = Self::open_file(file_path)?;
        let mut wkt_string = String::new();
        match file.read_to_string(&mut wkt_string) {
            Ok(_) => {}
            Err(e) => {
                panic!("Error reading WKT file: {}", e);
            }
        }

        let geom: Geometry = Wkt::from_str(&wkt_string).unwrap().item.try_into().unwrap();
        let records: Vec<Coord> = match geom {
            Geometry::Polygon(polygon) => polygon.coords_iter().collect(),
            _ => vec![],
        };

        Ok(records)
    }

    #[tracing::instrument]
    async fn insert_regions(&self, data: &[Region]) -> Result<()> {
        for region in data.iter() {
            sqlx::query(QUERY_INSERT_REGIONS)
                .bind(region.id)
                .bind(region.geo.as_str())
                .bind(region.name.as_str())
                .bind(region.name_en.as_str())
                .execute(self.pool())
                .await
                .with_context(|| "Error inserting regions into the database: {}")?;
        }

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn fetch_regions(&self) -> Result<Vec<Region>> {
        let query = sqlx::query_as(QUERY_SELECT_REGIONS).fetch_all(self.pool());

        let mut regions: Vec<Region> = query
            .await
            .with_context(|| "Error querying regions from the database: {}")?;

        if regions.len() <= 1 {
            let data = Self::read_csv_file(FILE_PATH_CSV)?;
            self.insert_regions(&data).await?;
            regions.extend(data);
        }

        Ok(regions)
    }

    #[tracing::instrument(skip(self))]
    async fn fetch_borders(&self) -> Result<Vec<Coord>> {
        let borders = Self::read_wkt_file(FILE_PATH_WKT)?;
        Ok(borders)
    }
}

impl MapRepository for DataRepository {
    #[tracing::instrument(skip(self))]
    async fn get_data(&mut self) -> Result<Ukraine> {
        let borders = self.fetch_borders().await?;
        let regions = self.fetch_regions().await?;
        let ukraine = Ukraine::new(borders, regions, None);
        Ok(ukraine)
    }
}
