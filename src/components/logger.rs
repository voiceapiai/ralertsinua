use color_eyre::eyre::Result;
use crossterm::event::KeyEvent;
use log::LevelFilter;
use ratatui::{
    layout::Rect,
    prelude::*,
    widgets::Block,
};
use rust_i18n::t;
use std::sync::Arc;

use tokio::sync::mpsc::UnboundedSender;
use tui_logger::{TuiLoggerLevelOutput, TuiLoggerWidget, TuiWidgetState};

use super::Component;
use crate::{action::Action, config::ConfigService, layout::*, tui::Frame};

// #[derive(Debug, Default, Clone, Copy, Display, FromRepr, EnumIter)]

// #[derive(Debug)]
pub struct Logger {
    command_tx: Option<UnboundedSender<Action>>,
    #[allow(unused)]
    config: Arc<dyn ConfigService>,

    state: TuiWidgetState,
}

impl Logger {
    pub fn new(config: Arc<dyn ConfigService>) -> Self {
        Self {
            command_tx: Option::default(),
            config,
            state: TuiWidgetState::new().set_default_display_level(LevelFilter::Trace),
        }
    }
}

impl Component for Logger {
    fn display(&self) -> Result<String> {
        Ok("Logger".to_string())
    }

    fn placement(&self) -> (LayoutArea, Option<LayoutTab>) {
        (LayoutArea::Inner, Some(LayoutTab::Tab2))
    }

    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn handle_key_events(&mut self, key_event: KeyEvent) -> Result<Option<Action>> {
        match key_event.code {
            // Other handlers you could add here.
            _ => Ok(None),
        }
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: &Rect) -> Result<()> {
        let widget = TuiLoggerWidget::default()
            .block(Block::bordered().title(t!("views.Logger.title").to_string().light_blue()))
            .style_error(Style::default().fg(Color::Red))
            .style_debug(Style::default().fg(Color::Green))
            .style_warn(Style::default().fg(Color::Yellow))
            .style_trace(Style::default().fg(Color::Reset))
            .style_info(Style::default().fg(Color::Cyan))
            .output_separator(':')
            .output_timestamp(Some("%H:%M:%S".to_string()))
            .output_level(Some(TuiLoggerLevelOutput::Long))
            .output_target(true)
            .output_file(true)
            .output_line(true)
            .state(&self.state);

        f.render_widget(widget, *area);
        Ok(())
    }
}
