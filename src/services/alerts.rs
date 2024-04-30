use crate::{alerts::*, error::*, ukraine::*, DataRepository};
use async_trait::async_trait;
use color_eyre::eyre::{Context, Error, Result};
use std::sync::Arc;

#[async_trait]
pub trait AlertService: Sync + Send + core::fmt::Debug {
    async fn by_region(&self) -> Result<Box<String>>;
    async fn list(&self) -> Result<Vec<Alert>>;
    // fn get_last_alert_response(&self) -> &str;
    // async fn get(&self, id: i32) -> Result<Alert>;
}

#[derive(Debug, Clone)]
pub struct AlertServiceImpl {
    pub repository: Arc<dyn DataRepository>,
}

impl AlertServiceImpl {
    pub fn new(repository: Arc<dyn DataRepository>) -> Self {
        AlertServiceImpl {
            repository,
        }
    }
}

#[async_trait]
impl AlertService for AlertServiceImpl {
    async fn by_region(&self) -> Result<Box<String>> {
        Ok(self.repository.fetch_alerts_string().await?)
    }

    async fn list(&self) -> Result<Vec<Alert>> {
        Ok(self.repository.fetch_alerts().await?)
    }

    // async fn get(&self, id: i32) -> Result<Alert, CommonError> {
    //     self.repository
    //         .fe(id)
    //         .await
    //         .map_err(|e| -> CommonError { e.into() })
    // }
}
