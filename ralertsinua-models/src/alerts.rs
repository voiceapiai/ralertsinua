use delegate::delegate;
use getset::Getters;
use serde::Deserialize;
use time::{format_description::BorrowedFormatItem, OffsetDateTime};
use time_macros::format_description;

use crate::{Alert, AlertType};

/// "2024/05/06 10:02:45 +0000"
/// const LAST_UPDATED_AT_FORMAT: &str = "%Y/%m/%d %H:%M:%S %z";
const LAST_UPDATED_AT_FORMAT: &[BorrowedFormatItem] = format_description!(
    "[year]/[month]/[day] [hour]:[minute]:[second] [offset_hour \
    sign:mandatory][offset_minute]"
);

/// Custom deserializer needed for the `last_updated_at` field in response
mod with_custom_date_format {
    use super::*;
    use serde::de::*;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        OffsetDateTime::parse(s.as_str(), LAST_UPDATED_AT_FORMAT).map_err(Error::custom)
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Getters)]
struct Meta {
    #[get = "pub with_prefix"]
    #[serde(with = "with_custom_date_format")]
    last_updated_at: OffsetDateTime,
}

/// Alerts struct is a collection of alerts and various methods to filter and access these alerts.
#[derive(Debug, Deserialize, Clone, PartialEq, Getters)]
pub struct Alerts {
    // #[get_copy = "pub with_prefix"]
    alerts: Vec<Alert>,
    #[get = "pub with_prefix"]
    disclaimer: String,
    //  #[serde(flatten)]
    meta: Meta,
}

impl Alerts {
    delegate! {
        to self.alerts {
            pub fn iter(&self) -> std::slice::Iter<Alert>;
            pub fn len(&self) -> usize;
            pub fn is_empty(&self) -> bool;
        }

        to self.meta {
            pub fn get_last_updated_at(&self) -> &OffsetDateTime;
        }
    }

    pub fn get_alerts(&self) -> Vec<Alert> {
        self.alerts.clone()
    }

    pub fn get_alerts_by_alert_type(&self, alert_type: AlertType) -> Vec<Alert> {
        self.alerts
            .iter()
            .filter(|alert| alert.alert_type == alert_type)
            .cloned() // Add this line to clone the alerts
            .collect()
    }

    pub fn get_alerts_by_location_title(&self, location_title: &str) -> Vec<Alert> {
        self.alerts
            .iter()
            .filter(|alert| alert.location_title == location_title)
            .cloned()
            .collect()
    }

    pub fn get_alerts_by_location_type(&self, location_type: &str) -> Vec<Alert> {
        self.alerts
            .iter()
            .filter(|alert| alert.location_type == location_type)
            .cloned()
            .collect()
    }

    pub fn get_alerts_by_region(&self, oblast_title: &str) -> Vec<Alert> {
        self.alerts
            .iter()
            .filter(|alert| alert.location_oblast == oblast_title)
            .cloned()
            .collect()
    }

    pub fn get_alerts_by_region_uid(&self, oblast_uid: i32) -> Vec<Alert> {
        self.alerts
            .iter()
            .filter(|alert| {
                alert.location_oblast_uid.is_some()
                    && alert.location_oblast_uid.unwrap() == oblast_uid
            })
            .cloned()
            .collect()
    }

    pub fn get_alerts_by_location_uid(&self, location_uid: i32) -> Vec<Alert> {
        self.alerts
            .iter()
            .filter(|alert| alert.location_uid == location_uid)
            .cloned()
            .collect()
    }

    pub fn get_air_raid_alerts(&self) -> Vec<Alert> {
        self.get_alerts_by_alert_type(AlertType::AirRaid)
    }

    pub fn get_artillery_shelling_alerts(&self) -> Vec<Alert> {
        self.get_alerts_by_alert_type(AlertType::ArtilleryShelling)
    }

    pub fn get_urban_fights_alerts(&self) -> Vec<Alert> {
        self.get_alerts_by_alert_type(AlertType::UrbanFights)
    }

    pub fn get_nuclear_alerts(&self) -> Vec<Alert> {
        self.get_alerts_by_alert_type(AlertType::Nuclear)
    }

    pub fn get_chemical_alerts(&self) -> Vec<Alert> {
        self.get_alerts_by_alert_type(AlertType::Chemical)
    }
}

mod tests {

    #[test]
    fn test_alerts_deserialization() {
        use super::*;
        use crate::AlertType;
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
            ],
            "disclaimer": "If you use python try our official alerts_in_ua PiP package.",
            "meta": {
                "last_updated_at": "2024/05/06 10:02:45 +0000",
                "type": "full"
            }
        });

        let all: Alerts = serde_json::from_value(data).unwrap();
        assert_eq!(all.alerts.len(), 2);

        let alert = &all.alerts[0];
        assert_eq!(alert.id, 8757);
        assert_eq!(alert.location_title, "Луганська область");
        assert_eq!(alert.location_type, "oblast");
        assert_eq!(alert.location_oblast, "Луганська область");
        assert_eq!(alert.location_uid, 16);
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
