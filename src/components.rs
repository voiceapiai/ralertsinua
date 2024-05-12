// use cached::proc_macro::cached;
use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::layout::{Constraint::*, Layout, Rect};
use tokio::sync::mpsc::UnboundedSender;
use tracing::debug;

use crate::{
    action::Action,
    config::Config,
    error::AppError,
    layout::*,
    tui::{Event, Frame},
    utils::type_of,
};

pub mod fps;
pub mod header;
pub mod list;
pub mod logger;
pub mod map;

pub use fps::*;
pub use header::*;
pub use list::*;
pub use logger::*;
pub use map::*;

pub type Result<T> = miette::Result<T, AppError>;

// #[cached]
pub fn get_component_area(
    frame_size: Rect,
    cmp_name: String,
    cmp_area: LayoutArea,
) -> Result<Rect> {
    let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
    let [header_area, inner_area, footer_area] = vertical.areas(frame_size);

    let horizontal = Layout::horizontal([Min(0), Length(20)]);
    let [tabs_area, title_area] = horizontal.areas(header_area);

    let main = Layout::horizontal([Percentage(75), Percentage(25)]);
    let [left_area, right_area] = main.areas(inner_area);

    let area = match cmp_area {
        LayoutArea::Header => header_area,
        LayoutArea::Tabs => tabs_area,
        LayoutArea::Title => title_area,
        LayoutArea::Inner => inner_area,
        LayoutArea::Left => left_area,
        LayoutArea::Right => right_area,
        LayoutArea::Footer => footer_area,
        LayoutArea::Hidden => Rect::default(),
    };

    debug!(target:"app", "calculated area for component '{}' - {:?}", cmp_name, area);

    Ok(area)
}

pub trait WithPlacement {
    /// Get the placement of the component.
    fn placement(&self) -> &LayoutPoint;
    /// Get rectangle in current frame for the component based on its placement
    fn get_area(&self, frame_size: Rect) -> Result<Rect> {
        let cmp_name = type_of(self).to_string();
        let LayoutPoint(area, _) = self.placement().clone();
        get_component_area(frame_size, cmp_name, area)
    }
    /// Check if the component is visible based on current selected tab
    fn is_visible(&self, selected_tab: &LayoutTab) -> bool {
        let LayoutPoint(_, cmp_tab) = self.placement();
        match cmp_tab {
            Some(tab) => tab == selected_tab,
            None => true,
        }
    }
    /// Debug self message
    fn debug(&self) {
        debug!(target:"app", "initializing component {}", type_of(self));
    }
}

/// `Component` is a trait that represents a visual and interactive element of the user interface.
/// Implementors of this trait can be registered with the main application loop and will be able to receive events,
/// update state, and be rendered on the screen.
pub trait Component<'a>: WithPlacement {
    /// Register an action handler that can send actions for processing if necessary.
    ///
    /// # Arguments
    ///
    /// * `tx` - An unbounded sender that can send actions.
    ///
    /// # Returns
    ///
    /// * `Result<()>` - An Ok result or an error.
    #[allow(unused_variables)]
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        Ok(())
    }
    /// Register a configuration handler that provides configuration settings if necessary.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration settings.
    ///
    /// # Returns
    ///
    /// * `Result<()>` - An Ok result or an error.
    #[allow(unused_variables)]
    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        Ok(())
    }
    /// Initialize the component with a specified area if necessary.
    ///
    /// # Arguments
    ///
    /// * `area` - Rectangular area to initialize the component within.
    ///
    /// # Returns
    ///
    /// * `Result<()>` - An Ok result or an error.
    fn init(&mut self, area: Rect) -> Result<()> {
        Ok(())
    }
    /// Handle incoming events and produce actions if necessary.
    ///
    /// # Arguments
    ///
    /// * `event` - An optional event to be processed.
    ///
    /// # Returns
    ///
    /// * `Result<Option<Action>>` - An action to be processed or none.
    fn handle_events(&mut self, event: Option<Event>) -> Result<Option<Action>> {
        let r = match event {
            Some(Event::Key(key_event)) => self.handle_key_events(key_event)?,
            // Some(Event::Mouse(mvent)) => self.handle_mouse_events(mouse_event)?,
            _ => None,
        };
        Ok(r)
    }
    /// Handle key events and produce actions if necessary.
    ///
    /// # Arguments
    ///
    /// * `key` - A key event to be processed.
    ///
    /// # Returns
    ///
    /// * `Result<Option<Action>>` - An action to be processed or none.
    #[allow(unused_variables)]
    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        Ok(None)
    }
    /// Handle mouse events and produce actions if necessary.
    ///
    /// # Arguments
    ///
    /// * `mouse` - A mouse event to be processed.
    ///
    /// # Returns
    ///
    /// * `Result<Option<Action>>` - An action to be processed or none.
    #[allow(unused_variables)]
    fn handle_mouse_events(&mut self, mouse: MouseEvent) -> Result<Option<Action>> {
        Ok(None)
    }
    /// Update the state of the component based on a received action. (REQUIRED)
    ///
    /// # Arguments
    ///
    /// * `action` - An action that may modify the state of the component.
    ///
    /// # Returns
    ///
    /// * `Result<Option<Action>>` - An action to be processed or none.
    #[allow(unused_variables)]
    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        Ok(None)
    }
    /// Render the component on the screen. (REQUIRED)
    ///
    /// # Arguments
    ///
    /// * `f` - A frame used for rendering.
    /// * `area` - The area in which the component should be drawn.
    ///
    /// # Returns
    ///
    /// * `Result<()>` - An Ok result or an error.
    fn draw(&mut self, f: &mut Frame<'_>) -> Result<()>;
}
