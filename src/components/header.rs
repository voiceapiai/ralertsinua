use crossterm::event::KeyEvent;
use ratatui::{
    style::{Modifier, Style},
    text::Line,
    widgets::Tabs,
};
use tokio::sync::mpsc::UnboundedSender;

use super::{Component, Result, WithPlacement};
use crate::{action::Action, config::*, constants::*, layout::*, tui::Frame};

#[derive(Debug)]
pub struct Header<'a> {
    command_tx: Option<UnboundedSender<Action>>,
    #[allow(unused)]
    config: Config,
    placement: LayoutPoint,
    #[allow(unused)]
    tabs: Vec<Line<'a>>,
    selected_tab: LayoutTab,
}

impl<'a> Header<'a> {
    pub fn new() -> Self {
        Self {
            command_tx: Option::default(),
            config: Config::default(),
            placement: LayoutPoint(LayoutArea::Header, None),
            tabs: Vec::new(),
            selected_tab: LayoutTab::default(),
        }
    }
}

impl WithPlacement<'_> for Header<'_> {
    fn placement(&self) -> &LayoutPoint {
        &self.placement
    }
}

impl<'a> Component<'a> for Header<'a> {
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

    fn draw(&mut self, f: &mut Frame) -> Result<()> {
        let area = self.get_area(f.size())?;
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

        f.render_widget(widget, area);
        Ok(())
    }
}
