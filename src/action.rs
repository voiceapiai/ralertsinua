#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use crate::{alerts::*, ukraine::*};
use arrayvec::ArrayVec;
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
    Selected(usize),
    Fetch,
    // #[serde(skip)]
    // SetListItems(RegionArrayVec, String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionWithLifetime<'a> {
    SetListItems(&'a [Region], &'a [char]),
}
