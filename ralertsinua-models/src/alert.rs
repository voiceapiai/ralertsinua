use chrono::{DateTime, FixedOffset, Utc};
use serde::Deserialize;
use std::fmt::Debug;
use strum::Display;
use strum_macros;

#[derive(Deserialize, Debug)]
pub struct Alert {
    pub id: i32,
    pub location_title: String,
    pub location_type: String,
    pub started_at: String,
    pub finished_at: Option<String>,
    // #[serde(skip_serializing)]
    #[serde(with = "custom_date_format")]
    pub updated_at: DateTime<Utc>,
    pub alert_type: String,
    pub location_oblast: String,
    pub location_uid: String,
    pub notes: Option<String>,
    // #[serde(skip_serializing)]
    pub country: Option<String>,
    #[serde(default)]
    pub calculated: Option<bool>,
    pub location_oblast_uid: Option<i32>,
}

pub const DEFAULT_ALERTS_RESPONSE_STRING: &str = "NNNNNNNNNNNNNNNNNNNNNNNNNNN";

#[derive(Debug, Deserialize)]
pub struct AlertsResponseAll {
    pub alerts: Vec<Alert>,
}

#[derive(Debug, strum_macros::EnumProperty, strum_macros::AsRefStr, Display, Default)]
pub enum AlertStatus {
    /// Active
    #[strum(props(icon = "🜸", color = "red"))]
    A,
    /// Partially active
    #[strum(props(icon = "🌤", color = "yellow"))]
    P,
    /// No information
    #[strum(props(icon = "🌣", color = "blue"))]
    #[default]
    N,
    /// Loading
    #[strum(props(icon = "↻", color = "white"))]
    L,
}

impl From<char> for AlertStatus {
    fn from(c: char) -> Self {
        match c {
            'A' => AlertStatus::A,
            'P' => AlertStatus::P,
            'L' => AlertStatus::L,
            _ => AlertStatus::N,
        }
    }
}

#[allow(unused)]
fn parse_alert_date(date: &str) -> DateTime<FixedOffset> {
    DateTime::parse_from_rfc3339(date).unwrap()
}

mod custom_date_format {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use serde::{self, de::Error as sError, Deserialize, Deserializer};

    /// @see https://serde.rs/custom-date-format.html
    const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer).unwrap();
        let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(sError::custom)?;
        Ok(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
    }
}
