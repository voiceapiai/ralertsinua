use serde::{
    // de::{self, Deserializer, Visitor},
    Deserialize, Serialize,
};
use strum::Display;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Display, Deserialize)]
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
    Selected(Option<usize>),
    Fetch,
    SelectTab(usize),
    SelectRegion(i8),
    SetAlertsByRegion(String),
    SetRegionGeo(String),
}
