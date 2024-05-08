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
pub enum LocationType {
    #[default]
    #[strum(to_string = "oblast")]
    Region,
    #[strum(to_string = "hromada")]
    Hromada,
    #[strum(to_string = "city")]
    City,
}

pub mod into_location_type {
    use super::LocationType;
    use serde::de::*;
    use std::str::FromStr;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<LocationType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        LocationType::from_str(s.as_str()).map_err(Error::custom)
    }
}
