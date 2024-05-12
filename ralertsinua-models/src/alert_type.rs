#[allow(unused_imports)]
use miette::{IntoDiagnostic, Result};
use std::str::FromStr;

#[derive(
    Debug, Default, Clone, strum_macros::Display, strum_macros::EnumString, PartialEq,
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

impl serde::Serialize for AlertType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
impl<'de> serde::Deserialize<'de> for AlertType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        Self: FromStr,
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::from_str(&value).map_err(serde::de::Error::custom)
    }
}
