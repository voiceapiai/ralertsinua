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
