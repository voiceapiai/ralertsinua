use ratatui::{
    style::Color,
    widgets::canvas::{Painter, Shape},
};
use serde::*;
use std::error::Error;
use std::io;
use std::process;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    pub id: String,
    pub name: String,
}

/// Shape to draw a world map with the given color
#[derive(Debug, Clone, PartialEq)]
pub struct Ukraine {
    pub data: [(f64, f64); 2],
    pub color: Color,
}

impl Default for Ukraine {
    fn default() -> Self {
        let data = [
            (-92.32, 48.24),
            (-88.13, 48.92),
        ];
        Self {
            data,
            color: Color::Yellow
        }
    }
}

impl Shape for Ukraine {
    fn draw(&self, painter: &mut Painter) {
        for (x, y) in self.data {
            if let Some((x, y)) = painter.get_point(x, y) {
                painter.paint(x, y, self.color);
            }
        }
    }
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// counter
    pub counter: u8,

    pub records: Vec<Record>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            counter: 0,
            records: vec![],
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        let records = Self::read_csv_file(None).expect("Failed to read CSV");
        Self {
            running: true,
            counter: 0,
            records,
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

    pub fn read_csv_file(file_path: Option<&str>) -> Result<Vec<Record>, Box<dyn Error>> {
        let file_path = file_path.unwrap_or("data/ukraine.csv");
        use csv::ReaderBuilder;

        let mut records = vec![];
        let file = std::fs::File::open(file_path)?;
        let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

        for result in rdr.deserialize() {
            let record: Record = match result {
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
}
