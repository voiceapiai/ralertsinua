use std::sync::Arc;

use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ralertsinua_geo::*;
use ralertsinua_http::*;
use ralertsinua_models::*;
use ratatui::prelude::*;
use tokio::{
    sync::mpsc,
    time::{sleep, Duration},
};
use tracing::{error, info};

use crate::{
    action::Action,
    components::{
        fps::FpsCounter, header::Header, list::RegionsList, logger::Logger, map::Map,
        Component,
    },
    config::*,
    layout::*,
    mode::Mode,
    tui,
};

pub struct App {
    pub config: Arc<dyn ConfigService>,
    pub api_client: Arc<dyn AlertsInUaApi>,
    pub geo_client: Arc<dyn AlertsInUaGeo>,
    pub components: Vec<Box<dyn Component>>,
    /// View stack: The top (=front) of the stack is the view that is displayed
    // pub view_stack: VecDeque<Box<dyn Component>>,
    pub should_quit: bool,
    pub should_suspend: bool,
    pub mode: Mode,
    pub selected_tab: LayoutTab,
    pub last_tick_key_events: Vec<KeyEvent>,
}

impl App {
    pub fn new(
        config: Arc<dyn ConfigService>,
        api_client: Arc<dyn AlertsInUaApi>,
        geo_client: Arc<dyn AlertsInUaGeo>,
    ) -> Result<Self> {
        let header = Header::new(config.clone());
        let map = Map::new(config.clone(), geo_client.clone());
        let list = RegionsList::new(config.clone(), api_client.clone());
        let fps = FpsCounter::new(config.clone());
        let logger = Logger::new(config.clone());
        let mode = Mode::Map;
        let components: Vec<Box<dyn Component>> = vec![
            Box::new(header),
            Box::new(map),
            Box::new(list),
            Box::new(fps),
            Box::new(logger),
        ];
        // let tick_rate = std::time::Duration::from_secs(10);
        Ok(Self {
            config,
            api_client,
            geo_client,
            components,
            // view_stack: VecDeque::new(),
            should_quit: false,
            should_suspend: false,
            mode,
            selected_tab: LayoutTab::default(),
            last_tick_key_events: Vec::new(),
        })
    }

    pub async fn init(&mut self) -> Result<()> {
        let response: Alerts = self.api_client.get_active_alerts().await?;

        info!("fetch_alerts: total {} alerts", response.len());
        response.iter().for_each(|alert| {
            info!("fetch_alerts:alert {:?}", alert);
        });
        Ok(())
    }

    pub fn next_tab(&mut self) {
        self.selected_tab = self.selected_tab.next();
    }

    pub fn previous_tab(&mut self) {
        self.selected_tab = self.selected_tab.previous();
    }

    pub async fn run(&mut self) -> Result<()> {
        let (action_tx, mut action_rx) = mpsc::unbounded_channel();
        let periodic_action_tx = action_tx.clone();

        let mut tui = tui::Tui::new()?
            .tick_rate(self.config.tick_rate())
            .frame_rate(self.config.frame_rate());
        // tui.mouse(true);
        tui.enter()?;

        self.init().await?;

        // ---------------------------------------------------------------------
        // EXAMPLE PERIODIC
        // ---------------------------------------------------------------------
        // dispatch fetch action after 2 seconds
        tokio::spawn(async move {
            sleep(Duration::from_secs(2)).await;
            if let Err(err) = periodic_action_tx.send(Action::Fetch) {
                error!("App->run: Failed to send fetch action: {:?}", err);
            } else {
                info!("App->run: Sent fetch action");
            }
        });

        for component in self.components.iter_mut() {
            component.register_action_handler(action_tx.clone())?;
        }

        for component in self.components.iter_mut() {
            component.init(tui.size()?).await?;
            info!("Initialized component {}", component.display()?);
        }

        loop {
            if let Some(e) = tui.next().await {
                match e {
                    tui::Event::Quit => action_tx.send(Action::Quit)?,
                    tui::Event::Tick => action_tx.send(Action::Tick)?,
                    tui::Event::Render => action_tx.send(Action::Render)?,
                    tui::Event::Resize(x, y) => action_tx.send(Action::Resize(x, y))?,
                    tui::Event::Key(key_event) => {
                        info!("Got key event: {key_event:?}");
                        match key_event.code {
                            KeyCode::Char('q') => {
                                action_tx.send(Action::Quit)?;
                            }
                            KeyCode::Char('c') | KeyCode::Char('C') => {
                                if key_event.modifiers == KeyModifiers::CONTROL {
                                    action_tx.send(Action::Quit)?;
                                }
                            }
                            KeyCode::Down => {
                                action_tx.send(Action::SelectRegion(1))?;
                            }
                            KeyCode::Up => {
                                action_tx.send(Action::SelectRegion(-1))?;
                            }
                            KeyCode::Right => {
                                self.next_tab();
                                action_tx
                                    .send(Action::SelectTab(self.selected_tab as usize))?;
                            }
                            KeyCode::Left => {
                                self.previous_tab();
                                action_tx
                                    .send(Action::SelectTab(self.selected_tab as usize))?;
                            }
                            KeyCode::Char('u') => {
                                action_tx.send(Action::Fetch)?;
                            }
                            KeyCode::Char('l') => {
                                action_tx.send(Action::Locale)?;
                            }
                            KeyCode::Char('r') => {
                                action_tx.send(Action::Refresh)?;
                            }
                            KeyCode::Char('z') => {
                                action_tx.send(Action::Suspend)?;
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
                for component in self.components.iter_mut() {
                    if let Some(action) = component.handle_events(Some(e.clone()))? {
                        action_tx.send(action)?;
                    }
                }
            }

            while let Ok(action) = action_rx.try_recv() {
                if action != Action::Tick && action != Action::Render {
                    log::debug!("{action:?}");
                }
                match action {
                    Action::Tick => {
                        self.last_tick_key_events.drain(..);
                    }
                    Action::Quit => self.should_quit = true,
                    Action::Suspend => self.should_suspend = true,
                    Action::Resume => self.should_suspend = false,
                    Action::Locale => {
                        self.config.toggle_locale();
                        action_tx.send(Action::Refresh)?;
                    }
                    Action::Resize(w, h) => {
                        tui.resize(Rect::new(0, 0, w, h))?;
                        // FIXME
                        /* tui.draw(|f| {
                            for component in self.components.iter_mut() {
                                let r = component.draw(f, &f.size());
                                if let Err(e) = r {
                                    action_tx
                                        .send(Action::Error(format!("Failed to draw: {:?}", e)))
                                        .unwrap();
                                }
                            }
                        })?; */
                    }

                    Action::Render => {
                        let layout = tui.layout().clone();
                        tui.draw(|f| {
                            self.components
                                .iter_mut()
                                .filter(|c| {
                                    let LayoutPoint(_, cmp_tab) = c.placement();
                                    cmp_tab.is_none()
                                        || cmp_tab.unwrap() == self.selected_tab
                                })
                                .for_each(|component| {
                                    let LayoutPoint(cmp_area, _) = component.placement();
                                    let area = layout.get_area(cmp_area);
                                    let r = component.draw(f, &area);
                                    if let Err(e) = r {
                                        action_tx
                                            .send(Action::Error(format!(
                                                "Failed to draw: {:?}",
                                                e
                                            )))
                                            .unwrap();
                                    }
                                });
                        })?;
                    }
                    Action::Fetch => {
                        // let alerts_as = self.facade.get_air_raid_alert_statuses_by_region().await?;
                        // let tx_action = Action::SetAlertsByRegion(alerts_as.to_string());

                        // // info!("App->on:FetchAlerts->action_tx.send: {}", tx_action);
                        // action_tx.send(tx_action)?;
                        // action_tx.send(Action::Refresh)?;
                    }
                    _ => {}
                }
                for component in self.components.iter_mut() {
                    if let Some(action) = component.update(action.clone())? {
                        action_tx.send(action)?
                    };
                }
            }
            if self.should_suspend {
                tui.suspend()?;
                action_tx.send(Action::Resume)?;
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
