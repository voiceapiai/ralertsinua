use std::fmt;

use crossterm::event::KeyEvent;
use log::LevelFilter;
use ratatui::{prelude::*, widgets::Block};
use rust_i18n::t;
use tokio::sync::mpsc::UnboundedSender;
use tui_logger::{TuiLoggerLevelOutput, TuiLoggerWidget, TuiWidgetState};

use super::{Component, Result, WithPlacement};
use crate::{action::Action, config::*, layout::*, tui::Frame};

pub struct Logger<'a> {
    command_tx: Option<UnboundedSender<Action>>,
    placement: LayoutPoint,
    #[allow(unused)]
    config: Config,
    #[allow(unused)]
    title: Line<'a>,
    state: TuiWidgetState,
}

impl<'a> fmt::Debug for Logger<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Logger")
            .field("command_tx", &self.command_tx)
            .field("placement", &self.placement)
            .field("config", &self.config)
            .field("title", &self.title)
            // Format the `state` field as a string
            .field("state", &"TuiWidgetState")
            .finish()
    }
}

impl<'a> Logger<'a> {
    pub fn new() -> Self {
        Self {
            command_tx: Option::default(),
            placement: LayoutPoint(LayoutArea::Inner, Some(LayoutTab::Tab2)),
            config: Config::default(),
            title: Line::default(),
            state: TuiWidgetState::new().set_default_display_level(LevelFilter::Trace),
        }
    }
}

impl WithPlacement<'_> for Logger<'_> {
    fn placement(&self) -> &LayoutPoint {
        &self.placement
    }
}

impl<'a> Component<'a> for Logger<'a> {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn handle_key_events(&mut self, key_event: KeyEvent) -> Result<Option<Action>> {
        #[allow(clippy::match_single_binding)]
        match key_event.code {
            // TODO: Other handlers you could add here.
            _ => Ok(None),
        }
    }

    fn draw(&mut self, f: &mut Frame) -> Result<()> {
        let area = self.get_area(f.size())?;
        let widget = TuiLoggerWidget::default()
            .block(
                Block::bordered().title(t!("views.Logger.title").to_string().light_blue()),
            )
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

        f.render_widget(widget, area);
        Ok(())
    }
}
