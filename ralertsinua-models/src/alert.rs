use crate::{alert_type::*, LocationType};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none, DisplayFromStr};
use time::OffsetDateTime;

#[skip_serializing_none]
#[serde_as]
#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct Alert {
    pub id: i32,
    pub location_title: String,
    #[serde_as(as = "DisplayFromStr")] // Serialize with Display, deserialize with FromStr
    pub location_type: LocationType,
    #[serde(with = "time::serde::iso8601")]
    pub started_at: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub updated_at: OffsetDateTime,
    // #[serde(with = "time::serde::iso8601")]
    pub finished_at: Option<String>, // TODO: parse Option to OffsetDateTime
    #[serde_as(as = "DisplayFromStr")] // Serialize with Display, deserialize with FromStr
    pub alert_type: AlertType,
    pub location_oblast: String,
    #[serde(with = "into_int")]
    pub location_uid: i32,
    pub location_oblast_uid: i32,
    pub notes: Option<String>,
    #[serde(default)]
    pub country: Option<String>,
    #[serde(default)]
    pub calculated: Option<bool>,
}

pub mod into_int {
    use super::*;
    use serde::{de::*, Serializer};

    pub fn serialize<S>(value: &i32, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&value.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<i32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse::<i32>().map_err(Error::custom)
    }
}

impl Alert {
    pub fn get_alert_duration(&self) -> std::time::Duration {
        let now = OffsetDateTime::now_utc();
        let offset_duration = now - self.started_at;
        offset_duration.try_into().unwrap()
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

        let alert = serde_json::from_value(data);
        if alert.is_err() {
            let err = alert.err().unwrap();
            panic!("Failed to deserialize Alert: {:?}", err);
        }
        let alert: Alert = alert.unwrap();

        assert_eq!(alert.id, 8757);
        assert_eq!(alert.location_title, "Луганська область");
        assert_eq!(alert.location_type, LocationType::Oblast);
        assert_eq!(alert.location_oblast, "Луганська область");
        assert_eq!(alert.location_uid, 16);
        assert_eq!(alert.location_oblast_uid, 16);
        assert_eq!(alert.alert_type, AlertType::AirRaid);
        assert_eq!(alert.notes, None);
        assert_eq!(alert.country, None);
        assert_eq!(alert.calculated, None);
        assert_eq!(alert.started_at.unix_timestamp(), 1_649_090_739);
        assert_eq!(alert.updated_at.unix_timestamp(), 1_698_603_757);
        assert_eq!(alert.finished_at, None);
    }
}
