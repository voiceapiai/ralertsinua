use crate::data::DataRepository;
use crate::ukraine::Ukraine;
use std::{error::Error, fmt::Debug};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub counter: u8,
    pub ukraine: Ukraine,
    data_repository: DataRepository,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(data_repository: DataRepository) -> Self {
        Self {
            running: true,
            counter: 0,
            ukraine: Ukraine::new(vec![], vec![], None),
            data_repository,
        }
    }
    /// Initialize app data
    pub async fn init(&mut self) -> AppResult<()> {
        use crate::data::MapRepository;
        self.ukraine = self.data_repository.get_data().await?;
        Ok(())
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
}
