/// This module contains the implementation of the `DataRepository` struct and the `MapRepository` trait.
/// The `DataRepository` struct provides methods for interacting with a SQLite database and fetching data related to Ukraine.
/// The `MapRepository` trait defines the `get_data` method, which returns a future that resolves to a `Result` containing the data for Ukraine.
use crate::ukraine::{Region, RegionArrayVec, Ukraine};
use anyhow::*;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use std::{fs::File, future::Future, io::Read, result::Result::Ok, sync::Arc, vec};
use tracing::{error, info};

#[allow(unused)]
const FILE_PATH_CSV: &'static str = "data/ukraine.csv";
const FILE_PATH_WKT: &'static str = "data/ukraine.wkt";
const DB_PATH: &'static str = "data/ukraine.sqlite";
const QUERY_CREATE_TABLE: &'static str = "
CREATE TABLE IF NOT EXISTS regions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    a_id INTEGER NOT NULL,
    osm_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    name_en TEXT NOT NULL,
    geo TEXT,
    UNIQUE(a_id) ON CONFLICT IGNORE
);

INSERT INTO regions (osm_id,a_id,name,name_en) VALUES
(145279,29,'Автономна Республіка Крим','Autonomous Republic of Crimea'),
(142129,4,'Волинська область','Volyn Oblast'),
(181453,8,'Вінницька область','Vinnytsia Oblast'),
(203493,9,'Дніпропетровська область','Dnipropetrovsk Oblast'),
(143947,28,'Донецька область','Donetsk Oblast'),
(142491,10,'Житомирська область','Zhytomyr Oblast'),
(144979,11,'Закарпатська область','Zakarpattia Oblast'),
(143961,12,'Запорізька область','Zaporizhia Oblast'),
(144977,13,'Івано-Франківська область','Ivano-Frankivsk Oblast'),
(843733,31,'Київ','Kyiv'),
(142497,14,'Київська область','Kyiv Oblast'),
(203719,15,'Кіровоградська область','Kirovohrad Oblast'),
(143943,16,'Луганська область','Luhansk Oblast'),
(144761,27,'Львівська область','Lviv Oblast'),
(145271,17,'Миколаївська область','Mykolaiv Oblast'),
(145269,18,'Одеська область','Odesa Oblast'),
(182589,19,'Полтавська область','Poltava Oblast'),
(142473,5,'Рівненська область','Rivne Oblast'),
(3148729,30,'Севастополь','Sevastopol'),
(142501,20,'Сумська область','Sumy Oblast'),
(145051,21,'Тернопільська область','Ternopil Oblast'),
(142509,22,'Харківська область','Kharkiv Oblast'),
(142045,23,'Херсонська область','Kherson Oblast'),
(181485,3,'Хмельницька область','Khmelnytskyi Oblast'),
(182557,24,'Черкаська область','Cherkasy Oblast'),
(145053,26,'Чернівецька область','Chernivtsi Oblast'),
(142499,25,'Чернігівська область','Chernihiv Oblast');
";
const QUERY_SELECT_REGIONS: &'static str = "SELECT * FROM regions ORDER BY id";

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
    fn read_wkt_file(file_path: &str) -> Result<String> {
        let mut file = Self::open_file(file_path)?;
        let mut wkt_string = String::new();
        file.read_to_string(&mut wkt_string)?;

        Ok(wkt_string)
    }

    /* #[tracing::instrument]
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

    #[tracing::instrument(skip(self))]
    async fn fetch_regions(&self) -> Result<RegionArrayVec> {
        use arrayvec::ArrayVec;
        let regions: Vec<Region> = sqlx::query_as(QUERY_SELECT_REGIONS)
            .fetch_all(self.pool())
            .await
            .with_context(|| "Error querying regions from the database: {}")?;

        Ok(ArrayVec::<Region, 27>::from_iter(regions))
    }

    #[tracing::instrument(skip(self))]
    async fn fetch_borders(&self) -> Result<String> {
        let borders = Self::read_wkt_file(FILE_PATH_WKT)?;
        Ok(borders)
    }
}

impl MapRepository for DataRepository {
    #[tracing::instrument(skip(self))]
    async fn get_data(&mut self) -> Result<Ukraine> {
        let borders = self.fetch_borders().await?;
        let regions = self.fetch_regions().await?;
        let ukraine = (Ukraine::new(borders, regions));
        Ok(ukraine)
    }
}
