#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use crate::{alerts::*, ukraine::*};
use serde::{
    de::{self, Deserializer, Visitor},
    Deserialize, Serialize,
};
use std::{fmt, string::ToString};
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
    Select(i8),
    SetAlertsByRegion(String),
    SetRegionGeo(String),
}
