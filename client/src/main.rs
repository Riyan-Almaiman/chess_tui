use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io::{self, Read, Write},
    net::TcpStream,
    sync::{Arc, Mutex},
    time::Duration,
    thread,
};
use crossterm::event::KeyCode;
use tui::{backend::CrosstermBackend, Terminal};

use serde::{Serialize, Deserialize};
use crate::pieces::Color;

#[derive(Serialize, Deserialize)]
pub enum NetMessage {
    AssignColor(Color),
    Move { from: (usize, usize), to: (usize, usize) },
}
mod app;
mod board;
mod pieces;
mod ui;

fn main() -> Result<(), io::Error> {
    let stream = TcpStream::connect("127.0.0.1:8080")?;
    stream.set_nonblocking(true)?;
    let stream = Arc::new(Mutex::new(stream));

    let app = Arc::new(Mutex::new(app::App::new(Color::White)));
    {
        let mut a = app.lock().unwrap();
        a.compute_all_moves();
        a.update_game_state();
    }

    {
        let stream = stream.clone();
        let app = app.clone();

        use std::io::BufRead;

        thread::spawn(move || {
            let stream = stream.lock().unwrap().try_clone().unwrap();
            let reader = std::io::BufReader::new(stream);

            for line in reader.lines() {
                if let Ok(line) = line {
                    if let Ok(msg) = serde_json::from_str::<NetMessage>(&line) {
                        match msg {
                            NetMessage::Move { from, to } => {
                                let mut app = app.lock().unwrap();

                                if app.last_sent_move == Some((from, to)) {
                                    app.last_sent_move = None;
                                    continue;
                                }

                                app.apply_network_move(from, to);
                            }

                            NetMessage::AssignColor(color) => {
                                let mut app = app.lock().unwrap();
                                app.player_color = color;
                            }
                        }
                    }
                }
            }
        });
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        {
            let app = app.lock().unwrap();
            terminal.draw(|f| {
                ui::draw(f, &app);
            })?;
        }

        if crossterm::event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                let mut app = app.lock().unwrap();

                if key.code == KeyCode::Char('q') {
                    break;
                }

                match key.code {
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if app.current_turn == app.player_color {
                            if let Some((from, to)) = app.handle_selection() {
                                app.last_sent_move = Some((from, to));

                                let msg = NetMessage::Move { from, to };
                                let encoded = serde_json::to_string(&msg).unwrap() + "\n";

                                let mut stream = stream.lock().unwrap();
                                let _ = stream.write_all(encoded.as_bytes());
                            }
                        }
                    }

                    KeyCode::Left => if app.cursor_x > 0 { app.cursor_x -= 1; },
                    KeyCode::Right => if app.cursor_x < 7 { app.cursor_x += 1; },
                    KeyCode::Up => if app.cursor_y > 0 { app.cursor_y -= 1; },
                    KeyCode::Down => if app.cursor_y < 7 { app.cursor_y += 1; },

                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}