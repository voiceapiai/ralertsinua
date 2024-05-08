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
    SelectRegion(i32),
    Selected(Option<AirRaidAlertOblastStatus>),
    Fetch,
}
