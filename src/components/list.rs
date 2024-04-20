use super::{Component, Frame};
use crate::{
    action::Action,
    config::{Config, KeyBindings},
    constants::*,
    data::DataRepository,
    ukraine::Ukraine,
};
use async_trait::async_trait;
use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Alignment, Constraint, Layout},
    prelude::*,
    style::{Color, Modifier, Style, Stylize},
    symbols::Marker,
    widgets::{canvas::*, Block, Borders, List as ListWidget},
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::Duration};
use tokio::sync::mpsc::UnboundedSender;

#[derive(Debug)]
pub struct List {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    data_repository: DataRepository,
    ukraine: Ukraine,
}

impl List {
    pub fn new(data_repository: DataRepository) -> Self {
        Self {
            command_tx: None,
            config: Config::default(),
            data_repository,
            ukraine: Ukraine::default(),
        }
    }
}

#[async_trait]
impl Component for List {
    fn display(&mut self) -> Result<String> {
        Ok("List".to_string())
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
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        // In addition to `List::new`, any iterator whose element is convertible to `ListItem` can be collected into `List`.
        let widget = ListWidget::new(self.ukraine.get_list_items().clone())
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

        f.render_stateful_widget(widget, area, &mut self.ukraine.list_state().clone());
        Ok(())
    }

    fn handle_key_events(&mut self, key_event: KeyEvent) -> Result<Option<Action>> {
        match key_event.code {
            // Counter handlers
            KeyCode::Down => {
                self.ukraine.next();
                let action = Action::Selected(self.ukraine.list_state().selected().unwrap());
                Ok(Some(action))
            }
            KeyCode::Up => {
                self.ukraine.previous();
                let action = Action::Selected(self.ukraine.list_state().selected().unwrap());
                Ok(Some(action))
            }
            // Other handlers you could add here.
            _ => Ok(None),
        }
    }
}
