use ralertsinua_geo::{CountryBoundary, Location};
use ralertsinua_models::*;
use serde::{
    // de::{self, Deserializer, Visitor},
    Deserialize,
    Serialize,
};
use strum::{Display, VariantNames};

#[derive(Debug, Clone, PartialEq, Serialize, Display, VariantNames, Deserialize)]
pub enum Action {
    Tick,
    Render,
    Resize(u16, u16),
    Suspend,
    Resume,
    Quit,
    Refresh,
    Error(String),
    Help,
    Online(bool),
    Locale,
    SelectTab(usize),
    SelectLocationByUid(Option<usize>),
    FetchGeo,
    GetLocations([Location; 27]),
    GetBoundaries(CountryBoundary),
    FetchActiveAlerts,
    GetActiveAlerts(Alerts),
    FetchAirRaidAlertOblastStatuses,
    GetAirRaidAlertOblastStatuses(AirRaidAlertOblastStatuses),
}
