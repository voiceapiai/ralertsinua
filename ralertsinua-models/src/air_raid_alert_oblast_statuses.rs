use miette::Result;
use serde::{Deserialize, Serialize};

use crate::{AirRaidAlertOblastStatus, AlertStatus, ModelError, REGIONS_DATA};

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct AirRaidAlertOblastStatuses {
    #[serde(skip)]
    raw_data: String,
    #[serde(with = "deserialize_oblast_statuses")]
    oblast_statuses: Vec<AirRaidAlertOblastStatus>,
}

/// Custom deserializer from char string to AirRaidAlertOblastStatuses
pub mod deserialize_oblast_statuses {
    use super::*;
    use serde::{de::*, Serializer};

    pub fn serialize<S>(
        #[allow(clippy::ptr_arg)] statuses: &Vec<AirRaidAlertOblastStatus>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut _s = String::new();
        statuses.iter().for_each(|s| {
            let c: char = s.status().to_string().chars().next().unwrap();
            _s.push(c);
        });
        serializer.serialize_str(&_s)
    }

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
        Self::new(String::from("OOOOOOOOOOOOOOOOOOOOOOOOOOO"), Some(true))
    }
}

impl AirRaidAlertOblastStatuses {
    pub fn iter(&self) -> std::slice::Iter<AirRaidAlertOblastStatus> {
        self.oblast_statuses.iter()
    }
    pub fn len(&self) -> usize {
        self.oblast_statuses.len()
    }
    pub fn is_empty(&self) -> bool {
        self.oblast_statuses.is_empty()
    }

    pub fn raw_data(&self) -> &str {
        &self.raw_data
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
            .map(|(i, (_, location_uid, name, name_en))| {
                let status = data_string.chars().nth(i).unwrap();
                AirRaidAlertOblastStatus::new(
                    *location_uid,
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
            raw_data: data.clone(),
            oblast_statuses: Self::from_string(data, oblast_level_only).unwrap(),
        }
    }

    pub fn get_all(&self) -> &[AirRaidAlertOblastStatus] {
        self.oblast_statuses.as_slice()
    }

    pub fn get(&self, idx: usize) -> Option<AirRaidAlertOblastStatus> {
        self.oblast_statuses.get(idx).cloned()
    }

    pub fn get_by_location_uid(
        &self,
        location_uid: i32,
    ) -> Option<AirRaidAlertOblastStatus> {
        self.oblast_statuses
            .iter()
            .find(|&os| os.location_uid == location_uid)
            .cloned()
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
