use alertsinua_tui::data::DataRepository;
use alertsinua_tui::app::{App, AppResult};
use alertsinua_tui::event::{Event, EventHandler};
use alertsinua_tui::handler::handle_key_events;
use alertsinua_tui::tui::Tui;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;

#[tokio::main]
async fn main() -> AppResult<()> {

    let pool = DataRepository::create_pool().await;
    let data_repository = DataRepository::new(pool);
    let mut app = App::new(data_repository);
    app.init().await?;

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next().await? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
