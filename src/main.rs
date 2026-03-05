use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use std::{io, time::Duration};
use tui::{
    Terminal,
    backend::CrosstermBackend,

};

use crate::app::{Color, Player};
mod app;
mod ui;
mod pieces;
fn main() -> Result<(), io::Error> {
    // setup terminal
    
    enable_raw_mode()?;


    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;


    let player = Player {
       color:  Color::White
    };


    let mut app = app::App::new(player);

    loop {
        terminal.draw(|f| {
            ui::draw(f, &app);
        })?;

        if crossterm::event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                if !app.handle_input(key.code) {
                    break;
                }
            }
        }
    }

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
