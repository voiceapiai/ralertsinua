use crate::data::DataRepository;
use crate::ukraine::Ukraine;
use anyhow::*;
use std::{fmt::Debug, sync::Arc};

/// Application.
#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub counter: u8,
    ukraine: Arc<Ukraine>,
    data_repository: DataRepository,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(data_repository: DataRepository) -> Self {
        Self {
            running: true,
            counter: 0,
            ukraine: Arc::new(Ukraine::default()),
            data_repository,
        }
    }
    /// Initialize app data
    pub async fn init(&mut self) -> Result<()> {
        use crate::data::MapRepository;
        let ukraine = self.data_repository.get_data().await?;
        self.ukraine = ukraine;
        Ok(())
    }

    pub fn ukraine(&self) -> &Ukraine {
        self.ukraine.as_ref()
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
