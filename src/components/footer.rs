use ratatui::{prelude::*, widgets::*};
#[allow(unused_imports)]
use rust_i18n::t;
use std::time::Instant;
use time::{format_description::well_known::Rfc2822, OffsetDateTime};
// use throbber_widgets_tui::{Throbber, ThrobberState, WhichUse, BRAILLE_SIX_DOUBLE};
use tokio::sync::mpsc::UnboundedSender;
// use tracing::debug;

use super::{Component, Result, WithPlacement};
use crate::{
    action::Action, config::*, constants::FULL_SCALE_WAR_START, layout::*, tui::Frame,
    tui_helpers::*,
};

#[derive(Debug, Clone)]
pub struct Footer<'a> {
    countdown: u32,
    last_updated: OffsetDateTime,
    // duration: Duration,
    app_start_time: Instant,
    app_frames: u32,
    app_fps: f64,

    render_start_time: Instant,
    render_frames: u32,
    render_fps: f64,
    command_tx: Option<UnboundedSender<Action>>,

    placement: LayoutPoint,
    #[allow(unused)]
    title: Line<'a>,
    // throbber_state: ThrobberState,
    #[allow(unused)]
    config: Config,
}

impl<'a> Footer<'a> {
    pub fn new() -> Self {
        Self {
            countdown: 60,
            last_updated: OffsetDateTime::now_utc(),
            // duration: Duration::from_millis(1645670400000),
            app_start_time: Instant::now(),
            app_frames: 0,
            app_fps: 0.0,
            render_start_time: Instant::now(),
            render_frames: 0,
            render_fps: 0.0,
            command_tx: Option::default(),

            placement: LayoutPoint(LayoutArea::Footer, None),
            title: Line::default(),
            // throbber_state: ThrobberState::default(),
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
        if self.countdown > 0 {
            self.countdown -= 1;
        } else {
            self.reset_timer()?;
        }
        self.render_frames += 1;
        let now = Instant::now();
        let elapsed = (now - self.render_start_time).as_secs_f64();
        if elapsed >= 1.0 {
            self.render_fps = self.render_frames as f64 / elapsed;
            self.render_start_time = now;
            self.render_frames = 0;
        }
        // self.throbber_state.calc_next();
        Ok(())
    }

    fn reset_timer(&mut self) -> Result<()> {
        self.countdown = *self.config.polling_interval();
        Ok(())
    }

    fn get_duration(&self) -> std::time::Duration {
        let now = OffsetDateTime::now_utc();
        let duration =
            now - OffsetDateTime::from_unix_timestamp(FULL_SCALE_WAR_START).unwrap();
        duration.try_into().unwrap()
    }
}

impl WithPlacement<'_> for Footer<'_> {
    fn placement(&self) -> &LayoutPoint {
        &self.placement
    }
}

impl<'a> Component<'a> for Footer<'a> {
    fn init(&mut self, size: Rect) -> Result<()> {
        self.debug();
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
            Action::Online(online) => {
                self.title = get_title_with_online_status("Satus", self.config.online())
                    .alignment(Alignment::Left);
            }
            Action::GetAirRaidAlertOblastStatuses(data) => {
                self.reset_timer()?;
            }
            Action::GetActiveAlerts(data) => {
                self.last_updated = data.get_last_updated_at();
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame) -> Result<()> {
        let mut area = self.get_area(f.size())?;
        // INFO: https://ratatui.rs/faq/#how-do-i-avoid-panics-due-to-out-of-range-calls-on-the-buffer
        area = area.intersection(area);
        let [left, right] = get_horizontal_area_split(area);

        let last_updated = self.last_updated.format(&Rfc2822).unwrap_or_default();
        let duration = format!(" ({})", dur::Duration::from_std(self.get_duration()));
        let status = Paragraph::new(Line::from(vec![
            "As of ".into(),
            last_updated.into(),
            duration.red().bold(),
        ]))
        .right_aligned();
        let countdown_ratio =
            self.countdown as f64 / *self.config.polling_interval() as f64;
        let progress = LineGauge::default()
            // .block(Block::bordered().title(self.title.clone()))
            .label(format!("Autorefresh in {} sec", self.countdown))
            .gauge_style(Style::default().yellow().on_blue().bold())
            .line_set(symbols::line::NORMAL)
            .ratio(countdown_ratio);

        f.render_widget(progress, left);
        f.render_widget(status, right);
        Ok(())
    }
}
