use ratatui::prelude::*;
use rust_i18n::t;
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};

use crate::constants::*;

#[derive(Debug, Default, Clone, Copy, Display, FromRepr, EnumIter, PartialEq)]
pub enum LayoutTab {
    #[default]
    #[strum(to_string = "tabs.home")]
    Tab1 = 0,
    #[strum(to_string = "tabs.logger")]
    Tab2 = 1,
}

impl LayoutTab {
    /// An iterator over the variants of LayoutTab
    pub fn into_iter() -> LayoutTabIter {
        LayoutTab::iter()
    }
    /// Get the previous tab, if there is no previous tab return the current tab.
    pub fn previous(self) -> Self {
        let current_index: usize = self as usize;
        let previous_index = current_index.saturating_sub(1);
        Self::from_repr(previous_index).unwrap_or(self)
    }
    /// Get the next tab, if there is no next tab return the current tab.
    pub fn next(self) -> Self {
        let current_index = self as usize;
        let next_index = current_index.saturating_add(1);
        Self::from_repr(next_index).unwrap_or(self)
    }
    /// Return tab's name as a styled `Line`
    pub fn title(self) -> Line<'static> {
        t!(&self.to_string())
            .to_string()
            .fg(*DEFAULT_COLOR)
            // .bg(self.palette().c900)
            .into()
    }
}

#[derive(Debug, Default, Clone, Hash, Display, FromRepr, EnumIter, PartialEq, Eq)]
pub enum LayoutArea {
    Header,
    Tabs,
    Title,
    Inner,
    Left,
    Right,
    Footer,
    #[default]
    Hidden,
}

#[derive(Debug, Clone)]
pub struct LayoutPoint(pub LayoutArea, pub Option<LayoutTab>);
