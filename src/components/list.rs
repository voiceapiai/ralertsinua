use crossterm::event::{KeyCode, KeyEvent};
use getset::*;
use ralertsinua_models::*;
use ratatui::{
    prelude::*,
    style::{Color, Modifier, Style},
    widgets::{Block, List, ListState},
};
use rust_i18n::t;
use tokio::sync::mpsc::UnboundedSender;
use tracing::info;

use super::{Component, Frame, Result, WithPlacement};
use crate::{action::Action, config::*, constants::*, draw::*, layout::*};

#[derive(Debug, Getters, MutGetters, Setters)]
pub struct LocationsList<'a> {
    command_tx: Option<UnboundedSender<Action>>,
    placement: LayoutPoint,
    #[allow(unused)]
    title: Line<'a>,
    config: Config,
    #[getset(get = "pub")]
    oblast_statuses: AirRaidAlertOblastStatuses,
    #[getset(get = "pub with_prefix")]
    list: List<'a>,
    #[getset(get = "pub", get_mut)]
    state: ListState,
    #[getset(get = "pub", get_mut)]
    last_selected: Option<usize>,
    selected_location_uid: i32,
}

impl<'a> LocationsList<'a> {
    pub fn new() -> LocationsList<'a> {
        Self {
            command_tx: None,
            placement: LayoutPoint(LayoutArea::Right, Some(LayoutTab::Tab1)),
            title: Line::default(),
            config: Config::default(),
            oblast_statuses: AirRaidAlertOblastStatuses::default(),
            list: List::default(),
            state: ListState::default(),
            last_selected: None,
            selected_location_uid: -1,
        }
    }

    /// Generate List Widget with ListItems of locations
    fn generate_list(&mut self, is_loading: bool) -> List<'a> {
        let locale = self.config.get_locale();
        let oblast_statuses = self.oblast_statuses();
        let items = oblast_statuses.iter().map(|item| {
            let text: &str = if locale.as_str() == "uk" {
                item.location_title()
            } else {
                item.location_title_en()
            };
            let is_selected = (item.location_uid) == self.selected_location_uid;
            Self::get_styled_line_by_status(text, item.status(), is_selected)
        });

        List::new(items)
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

impl WithPlacement for LocationsList<'_> {
    fn placement(&self) -> &LayoutPoint {
        &self.placement
    }
}

impl<'a> WithLineItems for LocationsList<'a> {}

impl<'a> Component<'a> for LocationsList<'a> {
    fn init(&mut self, size: Rect) -> Result<()> {
        self.debug();
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
            Action::GetAirRaidAlertOblastStatuses(data) => {
                self.oblast_statuses = data;
                self.list = self.generate_list(true);
            }
            Action::Refresh => {
                self.list = self.generate_list(false);
                info!("List->update->Action::Refresh: {}", action);
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame) -> Result<()> {
        let area = self.get_area(f.size())?;
        // let list: List<'a> = self.list.clone();
        let widget: List<'a> = self
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

        f.render_stateful_widget(widget, area, self.state_mut());
        Ok(())
    }

    fn handle_key_events(&mut self, key_event: KeyEvent) -> Result<Option<Action>> {
        match key_event.code {
            KeyCode::Down => {
                self.next();
                let selected = self.selected().unwrap();
                self.selected_location_uid = selected.location_uid;
                let action =
                    Action::SelectLocationByUid(Some(selected.location_uid as usize));
                Ok(Some(action))
            }
            KeyCode::Up => {
                self.previous();
                let selected = self.selected().unwrap();
                self.selected_location_uid = selected.location_uid;
                let action =
                    Action::SelectLocationByUid(Some(selected.location_uid as usize));
                Ok(Some(action))
            }
            KeyCode::Esc => {
                self.unselect();
                let action = Action::SelectLocationByUid(None);
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
        let list = LocationsList::new(Ukraine::new_arc(), );
        assert!(list.command_tx.is_none());
        assert_eq!(list.state, ListState::default());
        assert!(list.ukraine.read().unwrap().locations().is_empty() == true);
    } */
}
