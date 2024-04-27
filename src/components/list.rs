use super::{Component, Frame};
use crate::{
    action::Action,
    alerts::*,
    config::{self, get_locale, Locale},
    constants::*,
    data::DataRepository,
    tui::LayoutArea,
    ukraine::*,
};
use arrayvec::ArrayVec;
use async_trait::async_trait;
use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use delegate::delegate;
use getset::*;
use ratatui::{
    layout::{Alignment, Constraint, Layout},
    prelude::*,
    style::{Color, Modifier, Style, Stylize},
    symbols::Marker,
    widgets::{Block, Borders, List, ListItem, ListState},
};
use rust_i18n::t;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, str::FromStr, time::Duration};
use tokio::sync::mpsc::UnboundedSender;
use tracing::info;

impl<'a> From<&'a Region> for ratatui::text::Text<'a> {
    // In addition to `List::new`, any iterator whose element is convertible to `ListItem` can be collected into `List`.
    fn from(region: &'a Region) -> Self {
        Text::from(format!("{}: {}", region.id, region.name,))
    }
}

impl Region {
    /// Builds new `ListItem` from `Region` instance, based on references only
    pub fn to_list_item(
        &self,
        index: &usize,
        alert_status: &AlertStatus,
        locale: &Locale,
    ) -> ListItem<'static> {
        let icon: &str = alert_status.get_str("icon").unwrap();
        let color_str: &str = alert_status.get_str("color").unwrap();
        let color: Color = Color::from_str(color_str).unwrap();
        let text: &str = if *locale == Locale::uk {
            self.name.as_str()
        } else {
            self.name_en.as_str()
        };
        let list_item: ListItem = ListItem::new(format!("{} {}", icon, text)).style(color);

        match alert_status {
            AlertStatus::A => list_item
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::RAPID_BLINK),
            AlertStatus::P => list_item.add_modifier(Modifier::ITALIC),
            AlertStatus::L => list_item.add_modifier(Modifier::DIM),
            _ => list_item,
        }
    }
}

#[derive(Debug, Getters, MutGetters, Setters)]
pub struct RegionsList {
    command_tx: Option<UnboundedSender<Action>>,
    ukraine: Arc<RwLock<Ukraine>>,
    #[getset(get = "pub with_prefix")]
    list: List<'static>,
    #[getset(get = "pub", get_mut)]
    state: ListState,
    #[getset(get = "pub", get_mut)]
    last_selected: Option<usize>,
}

impl RegionsList {
    pub fn new(ukraine: Arc<RwLock<Ukraine>>) -> RegionsList {
        Self {
            command_tx: None,
            ukraine,
            list: List::default(),
            state: ListState::default(),
            last_selected: None,
        }
    }

    delegate! {
        to self.list {
            pub fn len(&mut self) -> usize;
        }

        to self.state {
            pub fn selected(&self) -> Option<usize>;
        }
    }

    /// Get List Widget with ListItems of regions
    fn list(&mut self, is_loading: bool) -> List<'static> {
        let ukraine = self.ukraine.read().unwrap();
        let regions = ukraine.regions();
        let alerts_as = ukraine.get_alerts();
        let locale = get_locale().unwrap();

        let items = regions.into_iter().enumerate().map(|(i, region)| {
            let region_a_s = if is_loading {
                AlertStatus::L
            } else {
                AlertStatus::from(alerts_as.chars().nth(i).unwrap_or('N'))
            };

            region.to_list_item(&i, &region_a_s, &locale)
        });

        List::new(items)
    }

    pub fn next(&mut self) {
        let lock = self.ukraine.read().unwrap();
        let i = match self.state.selected() {
            Some(i) => {
                if i >= lock.regions().len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => self.last_selected.unwrap_or(0),
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let lock = self.ukraine.read().unwrap();
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    lock.regions().len() - 1
                } else {
                    i - 1
                }
            }
            None => self.last_selected.unwrap_or(0),
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        let offset = self.state.offset();
        self.last_selected = self.state.selected();
        self.state.select(None);
        *self.state.offset_mut() = offset;
    }

    pub fn go_top(&mut self) {
        self.state.select(Some(0));
    }

    pub fn go_bottom(&mut self) {
        let lock = self.ukraine.read().unwrap();
        self.state.select(Some(lock.regions().len() - 1));
        // drop(lock);
    }
}

#[async_trait]
impl Component for RegionsList {
    fn display(&mut self) -> Result<String> {
        Ok("List".to_string())
    }

    fn placement(&mut self) -> LayoutArea {
        LayoutArea::Right_25
    }

    async fn init(&mut self, area: Rect) -> Result<()> {
        self.list = self.list(true);
        Ok(())
    }

    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    #[tracing::instrument(skip(self))]
    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Tick => {}
            Action::Refresh => {
                self.list = self.list(false);
                info!("List->update->Action::Refresh: {}", action);
            }
            Action::Fetch => {
                self.list = self.list(true);
                info!("List->update->Action::Fetch: {}", action);
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        let widget = self
            .list
            .clone()
            .block(
                Block::bordered()
                    .title(t!("views.List.title").to_string())
                    .title_alignment(Alignment::Center),
            )
            .style(Style::default().fg(Color::Cyan))
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(*SELECTED_STYLE_FG),
            )
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true);

        f.render_stateful_widget(widget, area, &mut self.state_mut());
        Ok(())
    }

    fn handle_key_events(&mut self, key_event: KeyEvent) -> Result<Option<Action>> {
        match key_event.code {
            KeyCode::Char('u') => {
                let action = Action::Fetch;
                Ok(Some(action))
            }
            KeyCode::Down => {
                self.next();
                let action = Action::Selected(self.state().selected());
                Ok(Some(action))
            }
            KeyCode::Up => {
                self.previous();
                let action = Action::Selected(self.state().selected());
                Ok(Some(action))
            }
            // Other handlers you could add here.
            _ => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_new() {
        let list = RegionsList::new(Ukraine::new_arc());
        assert!(list.command_tx.is_none());
        assert_eq!(list.state, ListState::default());
        assert!(list.ukraine.read().unwrap().regions().is_empty() == true);
        // match map.borders.try_from()
    }
}
