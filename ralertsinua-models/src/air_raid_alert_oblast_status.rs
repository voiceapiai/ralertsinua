use crate::AlertStatus;
use getset::Getters;
use serde::{Deserialize, Serialize};
use std::fmt;

// #[cfg(feature = "tui")] // TODO:
// use ratatui::{ style::{Color, Modifier, Stylize}, widgets::ListItem, };

#[derive(Debug, Clone, Getters, PartialEq, Serialize, Deserialize)]
pub struct AirRaidAlertOblastStatus {
    pub location_uid: i32,
    #[get = "pub"]
    location_title: String,
    #[get = "pub"]
    location_title_en: String,
    #[get = "pub"]
    status: AlertStatus,
}

impl fmt::Display for AirRaidAlertOblastStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use strum::EnumProperty;
        let icon: &str = self.status.get_str("icon").unwrap();
        let text = self.location_title.as_str();
        write!(f, "{} {}", icon, text)
    }
}

impl AirRaidAlertOblastStatus {
    pub fn new(
        location_uid: i32,
        location_title: String,
        location_title_en: String,
        status: char,
        oblast_level_only: Option<bool>,
    ) -> Self {
        let status: AlertStatus = if status == 'P' && oblast_level_only.unwrap() {
            AlertStatus::N
        } else {
            AlertStatus::from(status)
        };

        Self {
            location_uid,
            location_title,
            location_title_en,
            status,
        }
    }

    pub fn is_active_on_all_oblast(&self) -> bool {
        self.status == AlertStatus::A
    }

    pub fn is_partly_active(&self) -> bool {
        self.status == AlertStatus::P
    }

    pub fn is_no_alert(&self) -> bool {
        self.status == AlertStatus::N
    }
}
