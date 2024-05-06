use delegate::delegate;
use serde::Deserialize;

use crate::{AirRaidAlertOblastStatus, AlertStatus, ModelError, REGIONS_DATA};

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct AirRaidAlertOblastStatuses {
    #[serde(with = "deserialize_oblast_statuses")]
    oblast_statuses: Vec<AirRaidAlertOblastStatus>,
}

/// Custom deserializer from char string to AirRaidAlertOblastStatuses
pub mod deserialize_oblast_statuses {
    use super::*;
    use serde::de::*;

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Vec<AirRaidAlertOblastStatus>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?
            .trim_matches('"')
            .to_string();
        AirRaidAlertOblastStatuses::from_string(s, Some(true)).map_err(Error::custom)
    }
}

impl Default for AirRaidAlertOblastStatuses {
    fn default() -> Self {
        Self::new(String::from("NNNNNNNNNNNNNNNNNNNNNNNNNNN"), Some(true))
    }
}

impl AirRaidAlertOblastStatuses {
    delegate! {
        to self.oblast_statuses {
            pub fn iter(&self) -> std::slice::Iter<AirRaidAlertOblastStatus>;
            pub fn len(&self) -> usize;
            pub fn is_empty(&self) -> bool;
        }
    }

    /// Create a vec AirRaidAlertOblastStatuses from a string
    fn from_string(
        data_string: String,
        oblast_level_only: Option<bool>,
    ) -> Result<Vec<AirRaidAlertOblastStatus>, ModelError> {
        let data_string = data_string.trim_matches('"').to_string();
        let oblast_statuses: Vec<AirRaidAlertOblastStatus> = REGIONS_DATA
            .iter()
            .enumerate()
            .map(|(i, (_, _, name, name_en))| {
                let status = data_string.chars().nth(i).unwrap();
                AirRaidAlertOblastStatus::new(
                    name.to_string(),
                    name_en.to_string(),
                    status,
                    oblast_level_only,
                )
            })
            .collect::<Vec<AirRaidAlertOblastStatus>>();

        Ok(oblast_statuses)
    }

    pub fn new(data: String, oblast_level_only: Option<bool>) -> Self {
        Self {
            oblast_statuses: Self::from_string(data, oblast_level_only).unwrap(),
        }
    }

    pub fn get_all(&self) -> &[AirRaidAlertOblastStatus] {
        self.oblast_statuses.as_slice()
    }

    pub fn filter_by_status(&self, status: AlertStatus) -> Vec<AirRaidAlertOblastStatus> {
        self.oblast_statuses
            .iter()
            .filter(|&os| os.status() == &status)
            .cloned()
            .collect()
    }

    pub fn get_active_alert_oblasts(&self) -> Vec<AirRaidAlertOblastStatus> {
        self.filter_by_status(AlertStatus::A)
    }

    pub fn get_partly_active_alert_oblasts(&self) -> Vec<AirRaidAlertOblastStatus> {
        self.filter_by_status(AlertStatus::P)
    }

    pub fn get_no_alert_oblasts(&self) -> Vec<AirRaidAlertOblastStatus> {
        self.filter_by_status(AlertStatus::N)
    }
}
