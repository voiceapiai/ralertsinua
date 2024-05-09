use async_trait::async_trait;
use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use getset::*;
use ralertsinua_http::*;
use ralertsinua_models::*;
use ratatui::{
    prelude::*,
    style::{Color, Modifier, Style, Stylize},
    widgets::{Block, List, ListItem, ListState},
};
use rust_i18n::t;
use std::{str::FromStr, sync::Arc};
use tokio::sync::mpsc::UnboundedSender;
use tracing::info;

use super::{Component, Frame};
use crate::{action::Action, config::*, constants::*, layout::*};

#[derive(Debug, Getters, MutGetters, Setters)]
pub struct RegionsList {
    command_tx: Option<UnboundedSender<Action>>,
    config: Arc<dyn ConfigService>,
    api_client: Arc<dyn AlertsInUaApi>,
    #[getset(get = "pub")]
    oblast_statuses: AirRaidAlertOblastStatuses,
    #[getset(get = "pub with_prefix")]
    list: List<'static>,
    #[getset(get = "pub", get_mut)]
    state: ListState,
    #[getset(get = "pub", get_mut)]
    last_selected: Option<usize>,
}

impl RegionsList {
    pub fn new(
        config: Arc<dyn ConfigService>,
        api_client: Arc<dyn AlertsInUaApi>,
    ) -> RegionsList {
        Self {
            config,
            command_tx: None,
            api_client,
            oblast_statuses: AirRaidAlertOblastStatuses::default(),
            list: List::default(),
            state: ListState::default(),
            last_selected: None,
        }
    }

    /// Get List Widget with ListItems of locations
    fn list(&mut self, is_loading: bool) -> List<'static> {
        let locale = Locale::from_str(self.config.get_locale().as_str()).unwrap(); // TODO: improve
        let oblast_statuses = self.oblast_statuses();
        let items = oblast_statuses
            .iter()
            .map(|item| Self::to_list_item(item, &locale));

        List::new(items)
    }

    /// Builds new `ListItem` from `location`-like instance, based on references only
    pub fn to_list_item(
        item: &AirRaidAlertOblastStatus,
        locale: &Locale,
    ) -> ListItem<'static> {
        use strum::EnumProperty;

        let icon: &str = item.status().get_str("icon").unwrap();
        let color_str: &str = item.status().get_str("color").unwrap();
        let color: Color = Color::from_str(color_str).unwrap();
        let text: &str = if *locale == Locale::uk {
            item.location_title()
        } else {
            item.location_title_en()
        };
        let list_item: ListItem = ListItem::new(format!("{} {}", icon, text)).style(color);

        match item.status() {
            AlertStatus::A => list_item
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::RAPID_BLINK),
            AlertStatus::P => list_item.add_modifier(Modifier::ITALIC),
            AlertStatus::L => list_item.add_modifier(Modifier::DIM),
            _ => list_item,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.oblast_statuses().len() - 1 {
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
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.oblast_statuses().len() - 1
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
        self.state.select(Some(self.oblast_statuses().len() - 1));
        // drop(lock);
    }

    pub fn selected(&self) -> Option<AirRaidAlertOblastStatus> {
        match self.state.selected() {
            Some(i) => self.oblast_statuses().get(i),
            None => None,
        }
    }
}

#[async_trait]
impl Component for RegionsList {
    fn display(&self) -> Result<String> {
        Ok("List".to_string())
    }

    fn placement(&self) -> LayoutPoint {
        LayoutPoint(LayoutArea::Right, Some(LayoutTab::Tab1))
    }

    async fn init(&mut self, area: Rect) -> Result<()> {
        let result = self
            .api_client
            .get_air_raid_alert_statuses_by_location()
            .await?;
        self.oblast_statuses = result;
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
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: &Rect) -> Result<()> {
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

        f.render_stateful_widget(widget, *area, self.state_mut());
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
                let action = Action::Selected(self.selected());
                Ok(Some(action))
            }
            KeyCode::Up => {
                self.previous();
                let action = Action::Selected(self.selected());
                Ok(Some(action))
            }
            KeyCode::Esc => {
                self.unselect();
                let action = Action::Selected(None);
                Ok(Some(action))
            }
            // Other handlers you could add here.
            _ => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    /* #[test]
    fn test_map_new() {
        let list = RegionsList::new(Ukraine::new_arc(), );
        assert!(list.command_tx.is_none());
        assert_eq!(list.state, ListState::default());
        assert!(list.ukraine.read().unwrap().locations().is_empty() == true);
    } */
}
