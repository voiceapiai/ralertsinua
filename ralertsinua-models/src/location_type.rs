use std::str::FromStr;

#[derive(
    Debug,
    Default,
    Clone,
    strum_macros::Display,
    strum_macros::EnumString,
    strum_macros::VariantNames,
    PartialEq,
)]
#[strum(serialize_all = "lowercase")]
pub enum LocationType {
    #[default]
    #[strum(to_string = "oblast")]
    Oblast,
    #[strum(to_string = "hromada")]
    Hromada,
    #[strum(to_string = "city")]
    City,
}

impl serde::Serialize for LocationType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
impl<'de> serde::Deserialize<'de> for LocationType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        Self: FromStr,
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::from_str(&value).map_err(serde::de::Error::custom)
    }
}
