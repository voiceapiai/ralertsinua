use async_trait::async_trait;
use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use delegate::delegate;
use getset::*;
use ralertsinua_models::*;
use ratatui::{
    prelude::*,
    style::{Color, Modifier, Style, Stylize},
    widgets::{Block, List, ListItem, ListState},
};
use rust_i18n::t;
use std::str::FromStr;
use tokio::sync::mpsc::UnboundedSender;
use tracing::info;

use super::{Component, Frame};
use crate::{action::Action, config::*, constants::*, data::*, layout::*};

#[derive(Debug, Getters, MutGetters, Setters)]
pub struct RegionsList {
    command_tx: Option<UnboundedSender<Action>>,
    config: Arc<dyn ConfigService>,
    facade: Arc<dyn AlertsInUaFacade>,
    #[getset(get = "pub with_prefix")]
    list: List<'static>,
    #[getset(get = "pub", get_mut)]
    state: ListState,
    #[getset(get = "pub", get_mut)]
    last_selected: Option<usize>,
    last_alert_response: Option<String>,
}

impl RegionsList {
    pub fn new(
        config: Arc<dyn ConfigService>,
        facade: Arc<dyn AlertsInUaFacade>,
    ) -> RegionsList {
        Self {
            config,
            command_tx: None,
            facade,
            list: List::default(),
            state: ListState::default(),
            last_selected: None,
            last_alert_response: None,
        }
    }

    delegate! {

        to self.state {
            pub fn selected(&self) -> Option<usize>;
        }
    }

    fn get_last_alert_response(&self) -> &str {
        match self.last_alert_response {
            Some(ref response) => response.as_str(),
            None => DEFAULT_ALERTS_RESPONSE_STRING,
        }
    }

    /// Get List Widget with ListItems of regions
    fn list(&mut self, is_loading: bool) -> List<'static> {
        let alerts_as = self.get_last_alert_response();
        let locale = Locale::from_str(self.config.get_locale().as_str()).unwrap(); // TODO: improve

        let items = self.facade.regions().iter().enumerate().map(|(i, r)| {
            let region_a_s = if is_loading {
                AlertStatus::L
            } else {
                AlertStatus::from(alerts_as.chars().nth(i).unwrap_or('N'))
            };

            Self::to_list_item(r, &i, &region_a_s, &locale)
        });

        List::new(items)
    }

    /// Builds new `ListItem` from `Region` instance, based on references only
    pub fn to_list_item(
        r: &Region,
        index: &usize,
        alert_status: &AlertStatus,
        locale: &Locale,
    ) -> ListItem<'static> {
        use strum::EnumProperty;

        let icon: &str = alert_status.get_str("icon").unwrap();
        let color_str: &str = alert_status.get_str("color").unwrap();
        let color: Color = Color::from_str(color_str).unwrap();
        let text: &str = if *locale == Locale::uk {
            r.name.as_str()
        } else {
            r.name_en.as_str()
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

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.facade.regions().len() - 1 {
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
                    self.facade.regions().len() - 1
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
        self.state.select(Some(self.facade.regions().len() - 1));
        // drop(lock);
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
            Action::SetAlertsByRegion(alerts_as) => {
                self.last_alert_response = Some(alerts_as);
                self.list = self.list(true);
                // info!("List->update->Action::Fetch: {}", action);
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
                let seleced_region =
                    self.facade.regions()[self.state().selected().unwrap()].clone();
                let action = Action::Selected(self.state().selected());
                Ok(Some(action))
            }
            KeyCode::Up => {
                self.previous();
                let action = Action::Selected(self.state().selected());
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
        assert!(list.ukraine.read().unwrap().regions().is_empty() == true);
    } */
}
