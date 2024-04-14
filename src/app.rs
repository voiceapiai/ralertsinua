// use anyhow::{Context, Result};
use geo::{Coord, CoordsIter, Geometry};
// use geozero::{csv::*, error::*, wkt::*};
use crate::ukraine::*;
use std::{
    convert::TryInto, error::Error, fmt::Debug, fs::File, io::Read, process, result::Result, vec,
};
use wkt::Wkt;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub counter: u8,
    pub ukraine: Ukraine,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        let regions = Self::read_csv_file(None).expect("Failed to read CSV");
        let borders = Self::read_wkt_file(None).expect("Failed to read WKT");
        Self {
            running: true,
            counter: 0,
            ukraine: Ukraine::new(borders, regions, None),
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }

    pub fn read_csv_file(file_path: Option<&str>) -> Result<Vec<Region>, Box<dyn Error>> {
        let file_path = file_path.unwrap_or("data/ukraine.csv");
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

    pub fn read_wkt_file(file_path: Option<&str>) -> Result<Vec<Coord>, Box<dyn Error>> {
        use std::str::FromStr;
        let file_path = file_path.unwrap_or("data/ukraine.wkt");
        let mut file = File::open(file_path)?;
        let mut wkt_string = String::new();
        file.read_to_string(&mut wkt_string)?;

        let geom: Geometry = Wkt::from_str(&wkt_string).unwrap().item.try_into().unwrap();

        let records: Vec<Coord> = match geom {
            Geometry::Polygon(polygon) => polygon
                .coords_iter()
                .map(|c| Coord {
                    x: format!("{:.2}", c.x).parse().unwrap(),
                    y: format!("{:.2}", c.y).parse().unwrap(),
                })
                .collect(),
            _ => panic!("Not a polygon"),
        };

        Ok(records)
    }
}
