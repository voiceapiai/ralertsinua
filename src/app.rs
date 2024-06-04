use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
#[allow(unused_imports)]
use miette::{Context, WrapErr};
use ralertsinua_geo::*;
use ralertsinua_http::*;
use ralertsinua_models::*;
use ratatui::prelude::*;
use std::sync::Arc;
use tokio::{
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    time::{sleep, Duration},
};
#[allow(unused)]
use tracing::{debug, error, trace};

use crate::{action::*, components::*, config::*, error::*, layout::*, tui};

type Result<T> = miette::Result<T, AppError>;

pub struct App {
    action_tx: UnboundedSender<Action>,
    action_rx: UnboundedReceiver<Action>,
    pub config: Config,
    pub api_client: Arc<dyn AlertsInUaApi>,
    pub geo_client: Arc<dyn AlertsInUaGeo>,
    pub components: Vec<Box<dyn Component<'static>>>,
    /// View stack: The top (=front) of the stack is the view that is displayed
    // pub view_stack: VecDeque<Box<dyn Component>>, // TODO
    pub should_quit: bool,
    pub should_suspend: bool,
    pub selected_tab: LayoutTab,
    pub last_tick_key_events: Vec<KeyEvent>,
}

impl App {
    pub fn new(
        config: Config,
        api_client: Arc<dyn AlertsInUaApi>,
        geo_client: Arc<dyn AlertsInUaGeo>,
    ) -> Result<Self> {
        let (action_tx, action_rx) = mpsc::unbounded_channel();
        let header = Header::new();
        let map = Map::new();
        let list = LocationsList::new();
        let fps = Footer::new();
        let logger = Logger::new();
        let components: Vec<Box<dyn Component<'static>>> = vec![
            Box::new(header),
            Box::new(map),
            Box::new(list),
            Box::new(fps),
            Box::new(logger),
        ];
        Ok(Self {
            action_tx,
            action_rx,
            config,
            api_client,
            geo_client,
            components,
            should_quit: false,
            should_suspend: false,
            selected_tab: LayoutTab::default(),
            last_tick_key_events: Vec::new(),
        })
    }

    pub async fn init(&mut self) -> Result<()> {
        self.action_tx.send(Action::FetchGeo)?;
        self.action_tx
            .send(Action::FetchAirRaidAlertOblastStatuses)?;
        self.action_tx.send(Action::FetchActiveAlerts)?;
        Ok(())
    }

    pub fn next_tab(&mut self) {
        self.selected_tab = self.selected_tab.next();
    }

    pub fn previous_tab(&mut self) {
        self.selected_tab = self.selected_tab.previous();
    }

    pub fn selected_tab(&self) -> &LayoutTab {
        &self.selected_tab
    }

    pub async fn run(&mut self) -> Result<()> {
        let periodic_action_tx = self.action_tx.clone();
        let mut tui = tui::Tui::new()?
            .tick_rate(self.config.tick_rate())
            .frame_rate(self.config.frame_rate());
        // tui.mouse(true);
        tui.enter()?;

        // ---------------------------------------------------------------------
        self.init().await?;

        // ---------------------------------------------------------------------
        // EXAMPLE PERIODIC
        // ---------------------------------------------------------------------
        // dispatch fetch action after 2 seconds
        let interval = *self.config.polling_interval();
        debug!(target:"app", "init periodic fetch action every {} seconds", interval);
        tokio::spawn(async move {
            loop {
                sleep(Duration::from_secs_f32(interval as f32)).await;
                let _ = periodic_action_tx.send(Action::FetchAirRaidAlertOblastStatuses);
                let _ = periodic_action_tx.send(Action::FetchActiveAlerts);
            }
        });

        for component in self.components.iter_mut() {
            component.register_action_handler(self.action_tx.clone())?;
        }

        for component in self.components.iter_mut() {
            component.register_config_handler(self.config.clone())?;
        }

        for component in self.components.iter_mut() {
            component.init(tui.size()?)?;
        }

        loop {
            if let Some(e) = tui.next().await {
                match e {
                    tui::Event::Quit => self.action_tx.send(Action::Quit)?,
                    tui::Event::Tick => self.action_tx.send(Action::Tick)?,
                    tui::Event::Render => self.action_tx.send(Action::Render)?,
                    tui::Event::Resize(x, y) => {
                        self.action_tx.send(Action::Resize(x, y))?
                    }
                    tui::Event::Key(key_event) => match key_event.code {
                        KeyCode::Char('q') => {
                            self.action_tx.send(Action::Quit)?;
                        }
                        KeyCode::Char('c') | KeyCode::Char('C') => {
                            if key_event.modifiers == KeyModifiers::CONTROL {
                                self.action_tx.send(Action::Quit)?;
                            }
                        }
                        KeyCode::Right => {
                            self.next_tab();
                            self.action_tx
                                .send(Action::SelectTab(self.selected_tab as usize))?;
                        }
                        KeyCode::Left => {
                            self.previous_tab();
                            self.action_tx
                                .send(Action::SelectTab(self.selected_tab as usize))?;
                        }
                        KeyCode::Char('u') => {
                            // self.action_tx.send(Action::Fetch)?;
                        }
                        KeyCode::Char('l') => {
                            self.action_tx.send(Action::Locale)?;
                        }
                        KeyCode::Char('r') => {
                            self.action_tx.send(Action::Refresh)?;
                        }
                        KeyCode::Char('z') => {
                            self.action_tx.send(Action::Suspend)?;
                        }
                        _ => {}
                    },
                    _ => {}
                }
                for component in self.components.iter_mut() {
                    if let Some(action) = component.handle_events(Some(e.clone()))? {
                        self.action_tx.send(action)?;
                    }
                }
            }

            while let Ok(action) = self.action_rx.try_recv() {
                if action != Action::Tick && action != Action::Render {
                    debug!(target:"app_events", "received action: {}", action.to_string());
                }
                match action.clone() {
                    Action::Tick => {
                        self.last_tick_key_events.drain(..);
                    }
                    Action::Quit => self.should_quit = true,
                    Action::Suspend => self.should_suspend = true,
                    Action::Resume => self.should_suspend = false,
                    Action::Locale => {
                        self.config.toggle_locale();
                        self.action_tx.send(Action::Refresh)?;
                    }
                    Action::Resize(w, h) => {
                        tui.resize(Rect::new(0, 0, w, h))?;
                        // FIXME
                        /* tui.draw(|f| {
                            for component in self.components.iter_mut() {
                                let r = component.draw(f, &f.size());
                                if let Err(e) = r {
                                    self.action_tx
                                        .send(Action::Error(format!("Failed to draw: {:?}", e)))
                                        .unwrap();
                                }
                            }
                        })?; */
                    }
                    Action::Render => {
                        tui.draw(|f| {
                            let selected_tab = *self.selected_tab();
                            self.components
                                .iter_mut()
                                .filter(|c| c.is_visible(&selected_tab))
                                .for_each(|component| {
                                    let r = component.draw(f);
                                    if let Err(e) = r {
                                        self.action_tx
                                            .send(Action::Error(format!(
                                                "component failed to draw: {:?}",
                                                e
                                            )))
                                            .unwrap();
                                    }
                                });
                        })?;
                    }
                    Action::FetchGeo => {
                        let boundary = self.geo_client.boundary();
                        let locations = self.geo_client.locations();
                        debug!(target:"app", "fetch geo: total {} alerts", locations.len());
                        self.action_tx.send(Action::GetBoundaries(boundary))?;
                        self.action_tx.send(Action::GetLocations(locations))?;
                    }
                    Action::FetchActiveAlerts => {
                        let response: Alerts = self.api_client.get_active_alerts().await?;
                        debug!(target:"app", "get_active_alerts: total {} alerts", response.len());
                        self.action_tx.send(Action::GetActiveAlerts(response))?;
                    }
                    Action::FetchAirRaidAlertOblastStatuses => {
                        #[allow(clippy::bind_instead_of_map)]
                        let response: AirRaidAlertOblastStatuses = self
                            .api_client
                            .get_air_raid_alert_statuses_by_location()
                            .await
                            .and_then(|r| {
                                trace!(target: "app", "get_air_raid_alert_statuses_by_location: {}", r.raw_data());
                                Ok(r)
                            })
                            .map_err(|e| {
                                error!(target: "app", "error from API catched, possibly offline");
                                let _ = self.action_tx.send( Action::Error(e.to_string()));
                                let _ = self.action_tx.send( Action::Online(false));
                                e
                            })
                            .unwrap_or_default();
                        debug!(target:"app", "get_air_raid_alert_statuses_by_location: total {} alerts", response.len());
                        self.action_tx
                            .send(Action::GetAirRaidAlertOblastStatuses(response))?;
                    }
                    _ => {}
                }
                for component in self.components.iter_mut() {
                    if let Some(action) = component.update(action.clone())? {
                        self.action_tx.send(action)?
                    };
                }
            }
            if self.should_suspend {
                tui.suspend()?;
                self.action_tx.send(Action::Resume)?;
                tui = tui::Tui::new()?
                    .tick_rate(self.config.tick_rate())
                    .frame_rate(self.config.frame_rate());
                // tui.mouse(true);
                tui.enter()?;
            } else if self.should_quit {
                tui.stop()?;
                break;
            }
        }
        tui.exit()?;
        Ok(())
    }
}
