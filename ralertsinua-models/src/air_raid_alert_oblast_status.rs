use crate::AlertStatus;
use getset::Getters;
use std::fmt;

// #[cfg(feature = "tui")] // TODO:
// use ratatui::{ style::{Color, Modifier, Stylize}, widgets::ListItem, };

#[derive(Debug, Clone, Getters, PartialEq)]
pub struct AirRaidAlertOblastStatus {
    #[get = "pub"]
    location_title: String,
    #[get = "pub"]
    location_title_en: String,
    #[get = "pub"]
    status: AlertStatus,
}

impl fmt::Display for AirRaidAlertOblastStatus {
    #[cfg(not(feature = "tui"))]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use strum::EnumProperty;
        let icon: &str = self.status.get_str("icon").unwrap();
        let text = self.location_title.as_str();
        write!(f, "{} {}", icon, text)
    }

    #[cfg(feature = "tui")]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use strum::EnumProperty;
        let icon: &str = self.status.get_str("icon").unwrap();
        let text = self.location_title.as_str();
        write!(f, "{} {}", icon, text)
    }
}

impl AirRaidAlertOblastStatus {
    pub fn new(
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
