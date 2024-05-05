use ratatui::{
    layout::{Constraint, Layout, Rect},
    prelude::*,
};
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

#[derive(Debug, Default, Clone, Copy, Display, FromRepr, EnumIter, PartialEq)]
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

#[derive(Clone, Default)]
pub struct AppLayout {
    header_area: Rect,
    inner_area: Rect,
    footer_area: Rect,
    tabs_area: Rect,
    title_area: Rect,
    left_area: Rect,
    right_area: Rect,
}

impl AppLayout {
    pub fn new(fram_size: Rect) -> Self {
        use Constraint::*;
        let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
        let [header_area, inner_area, footer_area] = vertical.areas(fram_size);

        let horizontal = Layout::horizontal([Min(0), Length(20)]);
        let [tabs_area, title_area] = horizontal.areas(header_area);

        let main = Layout::horizontal([Percentage(75), Percentage(25)]);
        let [left_area, right_area] = main.areas(inner_area);

        Self {
            header_area,
            inner_area,
            footer_area,
            tabs_area,
            title_area,
            left_area,
            right_area,
        }
    }

    pub fn get_area(&self, area: LayoutArea) -> Rect {
        match area {
            LayoutArea::Header => self.header_area,
            LayoutArea::Tabs => self.tabs_area,
            LayoutArea::Title => self.title_area,
            LayoutArea::Inner => self.inner_area,
            LayoutArea::Left => self.left_area,
            LayoutArea::Right => self.right_area,
            LayoutArea::Footer => self.footer_area,
            LayoutArea::Hidden => Rect::default(),
        }
    }
}
