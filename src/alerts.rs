use arrayvec::ArrayString;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Deserialize, Debug)]
pub struct Alert {
    pub id: i32,
    pub location_title: String,
    pub location_type: String,
    pub started_at: String,
    pub finished_at: Option<String>,
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

#[derive(Debug, Deserialize)]
pub struct AlertsResponseAll {
    pub alerts: Vec<Alert>,
}

pub type AlertsResponseString = ArrayString<27>;


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, Display)]
pub enum AlertStatus {
    /// Active
    A,
    /// Partially active
    P,
    /// No information
    N,
}

mod custom_date_format {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use serde::{self, de::Error as sError, Deserialize, Deserializer};

    /// @see https://serde.rs/custom-date-format.html
    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer).unwrap();
        let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(sError::custom)?;
        Ok(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
    }
}

/// JSON string example to match later
pub const DEMO_ALERTS_RESPONSE: &str = r#"
{"alerts":[{"id":8757,"location_title":"Луганська область","location_type":"oblast","started_at":"2022-04-04T16:45:39.000Z","finished_at":null,"updated_at":"2023-10-29T18:22:37.357Z","alert_type":"air_raid","location_oblast":"Луганська область","location_uid":"16","notes":null,"country":null,"calculated":null,"location_oblast_uid":16},{"id":28288,"location_title":"Автономна Республіка Крим","location_type":"oblast","started_at":"2022-12-10T22:22:00.000Z","finished_at":null,"updated_at":"2023-10-29T16:56:12.340Z","alert_type":"air_raid","location_oblast":"Автономна Республіка Крим","location_uid":"29","notes":"Згідно інформації з Офіційних карт тривог","country":null,"calculated":null,"location_oblast_uid":29},{"id":71710,"location_title":"Мирівська територіальна громада","location_type":"hromada","started_at":"2024-04-18T05:43:26.000Z","finished_at":null,"updated_at":"..."}]}
"#;
