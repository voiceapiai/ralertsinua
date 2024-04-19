use alertsinua_tui::utils::*;
use alertsinua_tui::app::App;
use alertsinua_tui::data::*;
use alertsinua_tui::event::{Event, EventHandler};
use alertsinua_tui::handler::handle_key_events;
use alertsinua_tui::tui::{Tui, TuiResult};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;
use dotenv::dotenv;
use tracing::{info, Level};
use tracing_subscriber;

#[tokio::main]
async fn main() -> TuiResult<()> {
    dotenv().ok();
    initialize_logging();

    let pool = db_pool().await;
    let data_repository = DataRepository::new(pool);
    let mut app = App::new(data_repository);
    app.init().await?;
    info!("App initialized with LogLevel={}", Level::INFO);

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
