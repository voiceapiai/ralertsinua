use crate::data::DataRepository;
use crate::ukraine::Ukraine;
use anyhow::*;
use getset::{Getters, MutGetters, Setters};
use std::{fmt::Debug, sync::Arc};

/// Application.
#[derive(Debug, Getters, MutGetters, Setters)]
pub struct App {
    pub running: bool,
    #[getset(get = "pub", get_mut = "pub", set)]
    ukraine: Ukraine,
    #[getset(get = "pub")]
    arc: Arc<Ukraine>, // TODO: use arc if possible
    data_repository: DataRepository,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(data_repository: DataRepository) -> Self {
        Self {
            running: true,
            ukraine: (Ukraine::default()),
            arc: Arc::new(Ukraine::default()),
            data_repository,
        }
    }
    /// Initialize app data
    pub async fn init(&mut self) -> Result<()> {
        use crate::data::MapRepository;
        let ukraine = self.data_repository.get_data().await?;
        self.set_ukraine(ukraine);
        Ok(())
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn select_next(&mut self) {
        self.ukraine.next();
    }

    pub fn select_previous(&mut self) {
        self.ukraine.previous();
    }
}
