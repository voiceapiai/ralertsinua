use serde::Deserialize;
use std::fmt::Debug;
// use strum::{Display, EnumProperty, FromRepr};
use time::OffsetDateTime;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Alert {
    pub id: i32,
    pub location_title: String,
    pub location_type: String,
    #[serde(with = "time::serde::iso8601")]
    pub started_at: OffsetDateTime,
    #[serde(skip_serializing)]
    pub finished_at: Option<String>,
    #[serde(with = "time::serde::iso8601")]
    pub updated_at: OffsetDateTime,
    #[serde(with = "into_alert_type")]
    pub alert_type: AlertType,
    pub location_oblast: String,
    pub location_uid: String,
    pub notes: Option<String>,
    #[serde(default)]
    pub country: Option<String>,
    #[serde(default)]
    pub calculated: Option<bool>,
    pub location_oblast_uid: Option<i32>,
}

pub const DEFAULT_ALERTS_RESPONSE_STRING: &str = "NNNNNNNNNNNNNNNNNNNNNNNNNNN";

#[derive(Debug, Deserialize, PartialEq)]
pub struct AlertsResponseAll {
    pub alerts: Vec<Alert>,
}

#[derive(
    Debug,
    Default,
    strum_macros::EnumProperty,
    strum_macros::AsRefStr,
    strum_macros::Display,
)]
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

#[derive(
    Debug,
    Default,
    Deserialize,
    Clone,
    strum_macros::Display,
    strum_macros::EnumString,
    PartialEq,
)]
pub enum AlertType {
    #[default]
    #[strum(to_string = "air_raid")]
    AirRaid,
    #[strum(to_string = "artillery_shelling")]
    ArtilleryShelling,
}

mod into_alert_type {
    use super::AlertType;
    use serde::de::*;
    use std::str::FromStr;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<AlertType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let atype = AlertType::from_str(s.as_str()).unwrap();

        Ok(atype)
    }
}

mod tests {

    #[test]
    fn test_alert_deserialization() {
        use super::*;
        use serde_json::json;

        let data = json!({
            "alert_type": "air_raid",
            "calculated": null,
            "country": null,
            "finished_at": null,
            "id": 8757,
            "location_oblast": "Луганська область",
            "location_oblast_uid": 16,
            "location_title": "Луганська область",
            "location_type": "oblast",
            "location_uid": "16",
            "notes": null,
            "started_at": "2022-04-04T16:45:39.000Z",
            "updated_at": "2023-10-29T18:22:37.357Z"
        });

        let alert: Alert = serde_json::from_value(data).unwrap();

        assert_eq!(alert.id, 8757);
        assert_eq!(alert.location_title, "Луганська область");
        assert_eq!(alert.location_type, "oblast");
        assert_eq!(alert.location_oblast, "Луганська область");
        assert_eq!(alert.location_uid, "16");
        assert_eq!(alert.location_oblast_uid, Some(16));
        assert_eq!(alert.alert_type, AlertType::AirRaid);
        assert_eq!(alert.notes, None);
        assert_eq!(alert.country, None);
        assert_eq!(alert.calculated, None);
        assert_eq!(alert.started_at.unix_timestamp(), 1_649_090_739);
        assert_eq!(alert.updated_at.unix_timestamp(), 1_698_603_757);
        assert_eq!(alert.finished_at, None);
    }

    #[test]
    fn test_alerts_deserialization() {
        use super::*;
        use serde_json::json;

        let data = json!({
            "alerts": [
                {
                    "alert_type": "air_raid",
                    "calculated": null,
                    "country": null,
                    "finished_at": null,
                    "id": 8757,
                    "location_oblast": "Луганська область",
                    "location_oblast_uid": 16,
                    "location_title": "Луганська область",
                    "location_type": "oblast",
                    "location_uid": "16",
                    "notes": null,
                    "started_at": "2022-04-04T16:45:39.000Z",
                    "updated_at": "2023-10-29T18:22:37.357Z"
                }, {
                    "alert_type": "artillery_shelling",
                    "calculated": null,
                    "country": null,
                    "finished_at": null,
                    "id": 73992,
                    "location_oblast": "Дніпропетровська область",
                    "location_oblast_uid": 351,
                    "location_raion": "Нікопольський район",
                    "location_title": "Нікопольська територіальна громада",
                    "location_type": "hromada",
                    "location_uid": "351",
                    "notes": null,
                    "started_at": "2024-05-05T15:48:31.000Z",
                    "updated_at": "2024-05-05T15:48:31.818Z"
                }
            ]
        });

        let all: AlertsResponseAll = serde_json::from_value(data).unwrap();
        assert_eq!(all.alerts.len(), 2);

        let alert = &all.alerts[0];
        assert_eq!(alert.id, 8757);
        assert_eq!(alert.location_title, "Луганська область");
        assert_eq!(alert.location_type, "oblast");
        assert_eq!(alert.location_oblast, "Луганська область");
        assert_eq!(alert.location_uid, "16");
        assert_eq!(alert.location_oblast_uid, Some(16));
        assert_eq!(alert.alert_type, AlertType::AirRaid);
        assert_eq!(alert.notes, None);
        assert_eq!(alert.country, None);
        assert_eq!(alert.calculated, None);
        assert_eq!(alert.started_at.unix_timestamp(), 1_649_090_739);
        assert_eq!(alert.updated_at.unix_timestamp(), 1_698_603_757);
        assert_eq!(alert.finished_at, None);
    }
}
