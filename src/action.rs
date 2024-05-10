use geo::Polygon;
use ralertsinua_geo::Location;
use ralertsinua_models::*;
use serde::{
    // de::{self, Deserializer, Visitor},
    Deserialize,
    Serialize,
};
use strum::Display;

#[derive(Debug, Clone, PartialEq, Serialize, Display, Deserialize)]
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
    Locale,
    SelectTab(usize),
    SelectLocation(Option<usize>),
    FetchGeo,
    GetLocations([Location; 27]),
    GetBoundaries(Polygon),
    FetchActiveAlerts,
    GetActiveAlerts(Alerts),
    FetchAirRaidAlertOblastStatuses,
    GetAirRaidAlertOblastStatuses(AirRaidAlertOblastStatuses),
}
