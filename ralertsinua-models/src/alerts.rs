use color_eyre::eyre::Result;
use delegate::delegate;
use getset::Getters;
use serde::{Deserialize, Serialize};
use time::{format_description::BorrowedFormatItem, OffsetDateTime};
use time_macros::format_description;

#[allow(unused)]
use crate::{Alert, AlertType, LocationType, ModelError};

/// "2024/05/06 10:02:45 +0000"
/// const LAST_UPDATED_AT_FORMAT: &str = "%Y/%m/%d %H:%M:%S %z";
const LAST_UPDATED_AT_FORMAT: &[BorrowedFormatItem] = format_description!(
    "[year]/[month]/[day] [hour]:[minute]:[second] [offset_hour \
    sign:mandatory][offset_minute]"
);

/// Custom deserializer needed for the `last_updated_at` field in response
mod with_custom_date_format {
    use super::*;
    use serde::{de::*, Serializer};
    // use time::error::Format;

    pub fn serialize<S>(value: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // let _serr = Err(serde::ser::Error::custom( "path contains invalid UTF-8 characters", ));
        let s = value.format(LAST_UPDATED_AT_FORMAT).unwrap();
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let err = Error::custom("Invalid date format");
        let s = String::deserialize(deserializer).map_err(|_| err)?;
        OffsetDateTime::parse(s.as_str(), LAST_UPDATED_AT_FORMAT).map_err(Error::custom)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Getters)]
struct Meta {
    #[get = "pub with_prefix"]
    #[serde(with = "with_custom_date_format")]
    last_updated_at: OffsetDateTime,
}

/// Alerts struct is a collection of alerts and various methods to filter and access these alerts.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Getters)]
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
            .filter(|alert: &&Alert| alert.alert_type == alert_type)
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

    pub fn get_alerts_by_location_type(&self, location_type: LocationType) -> Vec<Alert> {
        self.alerts
            .iter()
            .filter(|alert| alert.location_type == location_type)
            .cloned()
            .collect()
    }

    pub fn get_alerts_by_location(&self, title: &str) -> Vec<Alert> {
        self.alerts
            .iter()
            .filter(|alert| alert.location_oblast == title)
            .cloned()
            .collect()
    }

    pub fn get_alerts_by_location_uid(&self, int_uid: i32) -> Vec<Alert> {
        self.alerts
            .iter()
            .filter(|alert| alert.location_uid == int_uid)
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
        // use time_macros::datetime;

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

        let alerts: Alerts = serde_json::from_value(data).unwrap();
        let alert1 = &alerts.alerts[0];
        let alert2 = &alerts.alerts[1];

        assert_eq!(alerts.iter().next(), Some(alert1));
        assert_eq!(alerts.len(), 2);
        assert_eq!(alerts.is_empty(), false);
        assert_eq!(
            alerts.meta.get_last_updated_at().unix_timestamp(),
            1_714_989_765
        );

        let expected_alert = alerts.get_air_raid_alerts();
        assert_eq!(expected_alert.len(), 1);
        assert_eq!(expected_alert[0].id, alert1.id);

        let expected_alert = alerts.get_artillery_shelling_alerts();
        assert_eq!(expected_alert.len(), 1);
        assert_eq!(expected_alert[0].id, alert2.id);

        let expected_alert = alerts.get_nuclear_alerts();
        assert_eq!(expected_alert.len(), 0);

        let expected_alert = alerts.get_chemical_alerts();
        assert_eq!(expected_alert.len(), 0);

        let expected_alert = alerts.get_urban_fights_alerts();
        assert_eq!(expected_alert.len(), 0);

        let expected_alert = alerts.get_alerts_by_alert_type(AlertType::UrbanFights);
        assert_eq!(expected_alert.len(), 0);

        let expected_alert = alerts.get_alerts_by_location_uid(16);
        assert_eq!(expected_alert.len(), 1);
        assert_eq!(expected_alert[0].id, alert1.id);

        let expected_alert = alerts.get_alerts_by_location_uid(351);
        assert_eq!(expected_alert.len(), 1);
        assert_eq!(expected_alert[0].id, alert2.id);

        let expected_alert = alerts.get_alerts_by_location_title("Луганська область");
        assert_eq!(expected_alert.len(), 1);
        assert_eq!(expected_alert[0].id, alert1.id);

        let expected_alert = alerts.get_alerts_by_location("Луганська область");
        assert_eq!(expected_alert.len(), 1);
        assert_eq!(expected_alert[0].id, alert1.id);

        let expected_alert = alerts.get_alerts_by_location_type(LocationType::Hromada);
        assert_eq!(expected_alert.len(), 1);
        assert_eq!(expected_alert[0].id, alert2.id);
    }
}
