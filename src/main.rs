// Crates
use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    Terminal
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

// Local Mods 
use fungitcide::ui;
use fungitcide::git_client;


fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        ui::draw_ui(f);
    })?;

    thread::sleep(Duration::from_secs(5));

    // restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    if false == true {
        git_client::commit::commit();
    }

    Ok(())
}
