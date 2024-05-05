use color_eyre::eyre::Result;
use crossterm::event::KeyEvent;
use ratatui::{
    layout::Rect,
    // prelude::*,
    style::{Modifier, Style},
    widgets::Tabs,
};
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender;

use super::Component;
use crate::{action::Action, config::ConfigService, constants::*, layout::*, tui::Frame};

#[derive(Debug)]
pub struct Header {
    command_tx: Option<UnboundedSender<Action>>,
    #[allow(unused)]
    config: Arc<dyn ConfigService>,

    selected_tab: LayoutTab,
}

impl Header {
    pub fn new(config: Arc<dyn ConfigService>) -> Self {
        Self {
            command_tx: Option::default(),
            config,
            selected_tab: LayoutTab::default(),
        }
    }
}

impl Component for Header {
    fn display(&self) -> Result<String> {
        Ok("Header".to_string())
    }

    fn placement(&self) -> LayoutPoint {
        LayoutPoint(LayoutArea::Tabs, None)
    }

    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn handle_key_events(&mut self, key_event: KeyEvent) -> Result<Option<Action>> {
        Ok(None)
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Tick => {}
            Action::SelectTab(tab) => {
                let tab = LayoutTab::from_repr(tab).unwrap();
                self.selected_tab = tab;
                // info!("List->update->Action::Fetch: {}", action);
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: &Rect) -> Result<()> {
        let titles = LayoutTab::into_iter().map(LayoutTab::title);
        let selected_tab_index = self.selected_tab as usize;
        let widget = Tabs::new(titles)
            .style(Style::default().fg(*DEFAULT_COLOR))
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::UNDERLINED)
                    .fg(*SELECTED_STYLE_FG),
            )
            .select(selected_tab_index)
            .padding("", "")
            .divider(" ");

        f.render_widget(widget, *area);
        Ok(())
    }
}
