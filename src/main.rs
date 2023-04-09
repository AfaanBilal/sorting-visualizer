/**
 * Sorting Visualizer
 *
 * @author Afaan Bilal (https://afaan.dev)
 * @link   https://github.com/AfaanBilal/sorting-visualizer
 */
use std::io;
use tui::backend::CrosstermBackend;
use tui::Terminal;
use sorting_visualizer::app::{App, AppResult};
use sorting_visualizer::event::{Event, EventHandler};
use sorting_visualizer::handler::handle_key_events;
use sorting_visualizer::tui::Tui;

fn main() -> AppResult<()> {
    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(1000);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
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
