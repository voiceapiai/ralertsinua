use serde::Deserialize;

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
    #[strum(to_string = "urban_fights")]
    UrbanFights,
    #[strum(to_string = "nuclear")]
    Nuclear,
    #[strum(to_string = "chemical")]
    Chemical,
}

pub mod into_alert_type {
    use super::AlertType;
    use serde::de::*;
    use std::str::FromStr;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<AlertType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        AlertType::from_str(s.as_str()).map_err(Error::custom)
    }
}
