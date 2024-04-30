use color_eyre::eyre::{ContextCompat, Error, Result};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::prelude::Rect;
#[allow(unused)]
use serde::{Deserialize, Serialize};
use tokio::{
    sync::mpsc,
    time::{sleep, Duration},
};
use tracing::{error, info};

use crate::{
    action::Action,
    cli::Cli,
    components::{fps::FpsCounter, list::RegionsList, map::Map, Component},
    config::*,
    data::*,
    mode::Mode,
    services::{alerts::AlertService, geo::GeoService},
    tui::{self, LayoutArea},
    ukraine::*,
};

pub struct App {
    pub config: Arc<dyn ConfigService>,
    pub alerts_service: Arc<dyn AlertService>,
    pub geo_service: Arc<dyn GeoService>,
    pub ukraine: Arc<RwLock<Ukraine>>,
    pub components: Vec<Box<dyn Component>>,
    pub should_quit: bool,
    pub should_suspend: bool,
    pub mode: Mode,
    pub last_tick_key_events: Vec<KeyEvent>,
}

impl App {
    pub fn new(
        config: Arc<dyn ConfigService>,
        ukraine: Arc<RwLock<Ukraine>>,
        alerts_service: Arc<dyn AlertService>,
        geo_service: Arc<dyn GeoService>,
    ) -> Result<Self> {
        let map = Map::new(ukraine.clone(), config.clone());
        let list = RegionsList::new(ukraine.clone(), config.clone());
        let fps = FpsCounter::new(ukraine.clone(), config.clone());
        let mode = Mode::Map;
        let components: Vec<Box<dyn Component>> =
            vec![Box::new(map), Box::new(list), Box::new(fps)];
        // let tick_rate = std::time::Duration::from_secs(10);
        Ok(Self {
            config,
            ukraine,
            alerts_service,
            geo_service,
            components,
            should_quit: false,
            should_suspend: false,
            mode,
            last_tick_key_events: Vec::new(),
        })
    }

    pub async fn init(&mut self) -> Result<()> {
        // TODO: if needed
        Ok(())
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

        /* for component in self.components.iter_mut() {
            component.register_config_handler(self.config.clone())?;
        } */

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
                                action_tx.send(Action::Select(1))?;
                            }
                            KeyCode::Up => {
                                action_tx.send(Action::Select(-1))?;
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
                        tui.draw(|f| {
                            for component in self.components.iter_mut() {
                                let r = component.draw(f, f.size());
                                if let Err(e) = r {
                                    action_tx
                                        .send(Action::Error(format!("Failed to draw: {:?}", e)))
                                        .unwrap();
                                }
                            }
                        })?;
                    }
                    Action::Render => {
                        tui.draw(|f| {
                            let [left, right] = tui::Tui::areas::<2>(f);
                            for (i, component) in self.components.iter_mut().enumerate() {
                                let area = match component.placement() {
                                    LayoutArea::Left_75 => left,
                                    LayoutArea::Right_25 => right,
                                };
                                let r = component.draw(f, area);
                                if let Err(e) = r {
                                    action_tx
                                        .send(Action::Error(format!("Failed to draw: {:?}", e)))
                                        .unwrap();
                                }
                            }
                        })?;
                    }
                    Action::Fetch => {
                        let alerts_as = self.alerts_service.by_region().await?;
                        let tx_action = Action::SetAlertsByRegion(alerts_as.to_string());

                        // info!("App->on:FetchAlerts->action_tx.send: {}", tx_action);
                        action_tx.send(tx_action)?;
                        action_tx.send(Action::Refresh)?;
                    }
                    Action::Selected(s) => {
                        match s {
                            Some(s) => {
                                let ukraine = self.ukraine.read().unwrap();
                                let region = ukraine.regions().get(s).unwrap();
                                let region_geo =
                                    self.geo_service.fetch_region_geo(region.osm_id).await?;
                                let tx_action = Action::SetRegionGeo(region_geo.to_string());

                                // info!("App->on:FetchAlerts->action_tx.send: {}", tx_action);
                                action_tx.send(tx_action)?;
                            }
                            None => {}
                        }
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
