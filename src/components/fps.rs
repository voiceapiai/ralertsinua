use color_eyre::eyre::Result;
use getset::Getters;
use ratatui::{layout::Offset, prelude::*, widgets::*};
use std::time::Instant;
use throbber_widgets_tui::{Throbber, ThrobberState, WhichUse, BRAILLE_SIX_DOUBLE};
use tokio::sync::mpsc::UnboundedSender;
// use tracing::debug;

use super::{Component, WithPlacement};
use crate::{action::Action, config::*, layout::*, tui::Frame};

#[derive(Debug, Clone, Getters)]
pub struct FpsCounter<'a> {
    app_start_time: Instant,
    app_frames: u32,
    app_fps: f64,

    render_start_time: Instant,
    render_frames: u32,
    render_fps: f64,
    command_tx: Option<UnboundedSender<Action>>,

    #[getset(get = "pub")]
    placement: LayoutPoint,
    #[allow(unused)]
    title: Line<'a>,
    throbber_state: ThrobberState,
    #[allow(unused)]
    config: Config,
}

impl<'a> FpsCounter<'a> {
    pub fn new() -> Self {
        Self {
            app_start_time: Instant::now(),
            app_frames: 0,
            app_fps: 0.0,
            render_start_time: Instant::now(),
            render_frames: 0,
            render_fps: 0.0,
            command_tx: Option::default(),

            placement: LayoutPoint(LayoutArea::Footer, None),
            title: Line::default(),
            throbber_state: ThrobberState::default(),
            config: Config::default(),
        }
    }

    fn app_tick(&mut self) -> Result<()> {
        self.app_frames += 1;
        let now = Instant::now();
        let elapsed = (now - self.app_start_time).as_secs_f64();
        if elapsed >= 1.0 {
            self.app_fps = self.app_frames as f64 / elapsed;
            self.app_start_time = now;
            self.app_frames = 0;
        }
        Ok(())
    }

    fn render_tick(&mut self) -> Result<()> {
        self.render_frames += 1;
        let now = Instant::now();
        let elapsed = (now - self.render_start_time).as_secs_f64();
        if elapsed >= 1.0 {
            self.render_fps = self.render_frames as f64 / elapsed;
            self.render_start_time = now;
            self.render_frames = 0;
        }
        self.throbber_state.calc_next();
        Ok(())
    }

    #[allow(unused)]
    fn dispatch_periodic_fetch_alerts(&self) -> Result<()> {
        Ok(())
    }
}

impl WithPlacement for FpsCounter<'_> {
    fn placement(&self) -> &LayoutPoint {
        &self.placement
    }
}

impl<'a> Component<'a> for FpsCounter<'a> {
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
            Action::Tick => {
                self.app_tick()?;
            }
            Action::Render => {
                self.render_tick()?;
            }
            Action::Refresh => {}
            Action::GetAirRaidAlertOblastStatuses(data) => {
                // debug!(target:"app", "FpsCounter->update: {:?}", action);
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame) -> Result<()> {
        let rects = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Min(0),
                Constraint::Length(2), // last row
            ])
            .split(f.size());
        let left = rects[1].offset(Offset { x: 1, y: 0 });
        let rect = rects[1].offset(Offset { x: 4, y: 0 });

        let s = format!(
            "{:.2} ticks per sec (app) {:.2} frames per sec (render)",
            self.app_fps, self.render_fps
        );
        let block =
            Block::default().title(block::Title::from(s.dim()).alignment(Alignment::Left));
        f.render_widget(block, rect);
        // Show "spinner"
        let throb = Throbber::default()
            .throbber_style(
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            )
            .throbber_set(BRAILLE_SIX_DOUBLE)
            .use_type(WhichUse::Spin);
        f.render_stateful_widget(throb, left, &mut self.throbber_state);
        Ok(())
    }
}
