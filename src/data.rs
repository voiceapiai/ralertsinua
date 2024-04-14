/// This module contains the implementation of the `DataRepository` struct and the `MapRepository` trait.
/// The `DataRepository` struct provides methods for interacting with a SQLite database and fetching data related to Ukraine.
/// The `MapRepository` trait defines the `get_data` method, which returns a future that resolves to a `Result` containing the data for Ukraine.
// use anyhow::{Context, Result};
// use geozero::{csv::*, error::*, wkt::*};
use crate::ukraine::*;
use geo::{Coord, CoordsIter, Geometry};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use std::{
    convert::TryInto, error::Error, fs::File, future::Future, io::Read, process, result::Result,
    vec,
};
use wkt::Wkt;

const FILE_PATH_CSV: &'static str = "data/ukraine.csv";
const FILE_PATH_WKT: &'static str = "data/ukraine.wkt";
const DB_PATH: &'static str = "data/ukraine.sqlite";

#[derive(Debug)]
pub struct DataRepository {
    /// The database pool.
    pool: SqlitePool,
}

pub trait MapRepository {
    fn get_data(&mut self) -> impl Future<Output = Result<Ukraine, Box<dyn Error>>> + Send;
}

impl DataRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_pool() -> SqlitePool {
        let conn: SqliteConnectOptions = SqliteConnectOptions::new()
            .filename(DB_PATH)
            .create_if_missing(true);

        let pool = match SqlitePool::connect_with(conn).await {
            Ok(pool) => pool,
            Err(e) => {
                eprintln!("Error connecting to sqlite database: {}", e);
                process::exit(1);
            }
        };
        match sqlx::query(
            "
            CREATE TABLE IF NOT EXISTS regions (
            id INTEGER PRIMARY KEY,
            geo TEXT NOT NULL,
            name TEXT NOT NULL,
            name_en TEXT NOT NULL
            )",
        )
        .execute(&pool)
        .await
        {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error creating sqlite table: {}", e);
                process::exit(1);
            }
        }
        // Return the pool
        pool
    }

    fn read_csv_file(file_path: &str) -> Result<Vec<Region>, Box<dyn Error>> {
        use csv::ReaderBuilder;

        let mut records = vec![];
        let file = File::open(file_path)?;
        let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

        for result in rdr.deserialize() {
            let record: Region = match result {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("Error reading CSV file: {}", e);
                    process::exit(1);
                }
            };
            records.push(record);
        }

        Ok(records)
    }

    fn read_wkt_file(file_path: &str) -> Result<Vec<Coord>, Box<dyn Error>> {
        use std::str::FromStr;
        let mut file = File::open(file_path)?;
        let mut wkt_string = String::new();
        file.read_to_string(&mut wkt_string)?;

        let geom: Geometry = Wkt::from_str(&wkt_string).unwrap().item.try_into().unwrap();

        let records: Vec<Coord> = match geom {
            Geometry::Polygon(polygon) => polygon.coords_iter().collect(),
            _ => panic!("Not a polygon"),
        };

        Ok(records)
    }

    async fn insert_regions(&self, data: &[Region]) -> Result<(), Box<dyn Error>> {
        for region in data.iter() {
            sqlx::query(
                "
            INSERT INTO regions (id, geo, name, name_en) VALUES (?, ?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
            geo = excluded.geo,
            name = excluded.name,
            name_en = excluded.name_en
            ",
            )
            .bind(region.id)
            .bind(region.geo.as_str())
            .bind(region.name.as_str())
            .bind(region.name_en.as_str())
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }

    async fn fetch_regions(&self) -> Result<Vec<Region>, Box<dyn Error>> {
        let query = sqlx::query_as("SELECT * FROM regions").fetch_all(&self.pool);

        let mut regions: Vec<Region> = query.await?;

        if regions.len() <= 1 {
            let data = Self::read_csv_file(FILE_PATH_CSV).expect("Failed to read CSV");
            self.insert_regions(&data).await?;
            regions.extend(data);
        }

        Ok(regions)
    }

    async fn fetch_borders(&self) -> Result<Vec<Coord>, Box<dyn Error>> {
        let borders = Self::read_wkt_file(FILE_PATH_WKT).expect("Failed to read WKT");
        Ok(borders)
    }
}

impl MapRepository for DataRepository {
    async fn get_data(&mut self) -> Result<Ukraine, Box<dyn Error>> {
        let borders = self.fetch_borders().await?;
        let regions = self.fetch_regions().await?;
        let ukraine = Ukraine::new(borders, regions, None);
        Ok(ukraine)
    }
}
