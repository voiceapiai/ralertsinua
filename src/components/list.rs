use super::{Component, Frame};
use crate::{
    action::Action,
    alerts::*,
    config::{Config, KeyBindings},
    constants::*,
    data::DataRepository,
    tui::LayoutArea,
    ukraine::{Region, Ukraine},
};
use async_trait::async_trait;
use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use getset::*;
use ratatui::{
    layout::{Alignment, Constraint, Layout},
    prelude::*,
    style::{Color, Modifier, Style, Stylize},
    symbols::Marker,
    widgets::{Block, Borders, List, ListItem, ListState},
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::sync::mpsc::UnboundedSender;
use tracing::info;

impl Region {
    pub fn to_list_item(&self, index: i8, alert_status: char) -> ListItem {
        use std::str::FromStr;
        let alert_status = AlertStatus::from(alert_status);
        let icon = alert_status.get_str("icon").unwrap();
        let color_str = alert_status.get_str("color").unwrap();
        let color = Color::from_str(color_str).unwrap();
        let list_item: ListItem = ListItem::new(format!("{} {}", icon, self.name)).style(color);

        match alert_status {
            AlertStatus::A => list_item
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::RAPID_BLINK),
            AlertStatus::P => list_item.add_modifier(Modifier::ITALIC),
            AlertStatus::N => list_item,
        }
    }
}

#[derive(Debug, Getters, MutGetters, Setters)]
pub struct RegionsList {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    data_repository: Arc<DataRepository>,
    ukraine: Ukraine,
    #[getset(get = "pub")]
    state: ListState,
    #[getset(get = "pub", get_mut = "pub")]
    last_selected: Option<usize>,
}

impl RegionsList {
    pub fn new(data_repository: Arc<DataRepository>) -> Self {
        Self {
            command_tx: None,
            config: Config::default(),
            data_repository,
            ukraine: Ukraine::default(),
            state: ListState::default(),
            last_selected: None,
        }
    }

    pub fn get_list_items(&self) -> Vec<ListItem> {
        self.ukraine
            .regions()
            .iter()
            .enumerate()
            .map(|(i, r)| {
                r.to_list_item(
                    i as i8,
                    self.ukraine.get_alerts().chars().nth(i).unwrap_or('N'),
                )
            })
            .collect()
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.ukraine.regions().len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => self.last_selected.unwrap_or(0),
        };
        self.state.select(Some(i));
        // info!("List->next, selected region: {:?}", i);
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.ukraine.regions().len() - 1
                } else {
                    i - 1
                }
            }
            None => self.last_selected.unwrap_or(0),
        };
        self.state.select(Some(i));
        // info!("List->previous, selected region: {:?}", i);
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
        self.state.select(Some(self.ukraine.regions().len() - 1));
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
        use crate::data::MapRepository;
        self.ukraine = self.data_repository.get_data().await?;
        Ok(())
    }

    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Tick => {}
            Action::FetchAlerts => {
                self.data_repository.fetch_alerts_short();
                info!("List->update: {:?}", action);
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        // In addition to `List::new`, any iterator whose element is convertible to `ListItem` can be collected into `List`.
        let widget = List::new(self.get_list_items())
            .block(
                Block::bordered()
                    .title("Regions")
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

        f.render_stateful_widget(widget, area, &mut self.state().clone());
        Ok(())
    }

    fn handle_key_events(&mut self, key_event: KeyEvent) -> Result<Option<Action>> {
        match key_event.code {
            KeyCode::Char('u') => {
                let action = Action::FetchAlerts;
                Ok(Some(action))
            }
            KeyCode::Down => {
                self.next();
                let action = Action::Selected(self.state().selected().unwrap());
                Ok(Some(action))
            }
            KeyCode::Up => {
                self.previous();
                let action = Action::Selected(self.state().selected().unwrap());
                Ok(Some(action))
            }
            // Other handlers you could add here.
            _ => Ok(None),
        }
    }
}
