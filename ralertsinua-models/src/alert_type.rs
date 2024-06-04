#[derive(
    Debug, Default, Clone, strum_macros::Display, strum_macros::EnumString, PartialEq,
)]
// coveralls-ignore-next-line
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
