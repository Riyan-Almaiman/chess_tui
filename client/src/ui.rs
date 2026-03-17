use std::fmt::format;
use tui::{
    Frame,
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::{App, GameState};
struct ScreenLayout {
    board: Rect,
    sidebar: Rect,
}
const BOARD_ROWS: u16 = 8;
const BOARD_COLS: u16 = 8;

// Terminal cells are taller than wide, so each square is 2x1
const CELL_ASPECT_WIDTH: u16 = 2;
const CELL_ASPECT_HEIGHT: u16 = 1;

const SIDEBAR_PERCENT: u16 = 20;
const BOARD_PERCENT: u16 = 80;

const INFO_PANEL_HEIGHT: u16 = 7;
const MOVES_PANEL_HEIGHT: u16 = 10;

const COLOR_LIGHT_SQUARE: Color = Color::Rgb(144, 134, 100);
const COLOR_DARK_SQUARE: Color = Color::Rgb(120, 70, 40);

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &App) {
    let size = f.size();
    let layout = main_layout(size);

    let board_with_margin = Rect {
        x: layout.board.x + 2,
        y: layout.board.y,
        width: layout.board.width - 2,
        height: layout.board.height - 1,
    };

    let final_board = calculate_board_size(board_with_margin);

    render_sidebar(f, layout.sidebar, app);
    draw_board(f, final_board, app);
    draw_labels(f, final_board, app);
}
fn draw_labels<B: Backend>(f: &mut Frame<B>, area: Rect, app: &App) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(area.width / 8); 8])
        .split(Rect {
            x: area.x + 2,
            y: area.y + area.height ,
            width: area.width - 3,
            height: 1,
        });

    let letters = if app.player_color == crate::pieces::Color::White {
        ["a", "b", "c", "d", "e", "f", "g", "h"]
    } else {
        ["h", "g", "f", "e", "d", "c", "b", "a"]
    };

    for (i, col) in cols.iter().enumerate() {
        let p = Paragraph::new(letters[i])
            .alignment(tui::layout::Alignment::Center);
        f.render_widget(p, *col);
    }

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(area.height / 8); 8])
        .split(Rect {
            x: area.x.saturating_sub(3),
            y: area.y,
            width: 2,
            height: area.height - 1,
        });

    for (i, row) in rows.iter().enumerate() {
        let label = if app.player_color == crate::pieces::Color::White {
            format!("{}", 8 - i)
        } else {
            format!("{}", i + 1)
        };        let p = Paragraph::new(label)
            .alignment(tui::layout::Alignment::Center);
        f.render_widget(p, *row);
    }
}
fn main_layout(area: Rect) -> ScreenLayout {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(BOARD_PERCENT),
            Constraint::Percentage(SIDEBAR_PERCENT),
        ])
        .split(area);

    ScreenLayout {
        board: chunks[0],
        sidebar: chunks[1],
    }
}

fn draw_board<B: Backend>(f: &mut Frame<B>, board_area: Rect, app: &App) {
    let row_h = board_area.height / BOARD_ROWS;
    let col_w = board_area.width / BOARD_COLS;

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(row_h); BOARD_ROWS as usize])
        .split(board_area);

    for (y, row) in rows.iter().enumerate() {
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(col_w); BOARD_COLS as usize])
            .split(*row);

        for (x, col) in cols.iter().enumerate() {
            draw_square(f, *col, x, y, app);
        }
    }
}

fn draw_square<B: Backend>(f: &mut Frame<B>, area: Rect, x: usize, y: usize, app: &App) {
    let logical_pos = if app.player_color == crate::pieces::Color::White {
        (y, x)
    } else {
        (7 - y, 7 - x)
    };
    let mut style = square_style(logical_pos.1, logical_pos.0);
    if let Some(pos) = app.selected {
        let moves_for_selected = app.current_valid_moves.iter().find(|m| m.position == pos);

        if let Some(piece_moves) = moves_for_selected {
            if piece_moves.moves.contains(&logical_pos) {
                style = style.bg(Color::Rgb(60, 60, 60));
            }
        }
    }

    if Some(logical_pos) == app.selected {
        style = style.bg(Color::Blue);
    } else if x == app.cursor_x && y == app.cursor_y && app.player_color == app.current_turn{
        style = style.bg(Color::Yellow);
    }

    let (board_y, board_x) = if app.player_color == crate::pieces::Color::White {
        (y, x)
    } else {
        (7 - y, 7 - x)
    };

    let piece = app.board.get_piece_at((board_y, board_x));
    if let Some(p) = piece {
        style = match p.color {
            crate::pieces::Color::White => style.fg(Color::White),
            crate::pieces::Color::Black => style.fg(Color::Black),
        };
    }

    let piece_char = piece.map_or("", |p| p.get_ui());

    let vertical_padding = (area.height.saturating_sub(1) / 2) as usize;
    let mut display_text = "\n".repeat(vertical_padding);
    display_text.push_str(piece_char);

    let para = Paragraph::new(display_text)
        .style(style)
        .alignment(tui::layout::Alignment::Center);

    f.render_widget(para, area);
}

fn square_style(x: usize, y: usize) -> Style {
    if (x + y) % 2 == 1 {
        Style::default().bg(COLOR_DARK_SQUARE)
    } else {
        Style::default().bg(COLOR_LIGHT_SQUARE)
    }
}
fn calculate_board_size(area: Rect) -> Rect {
    let max_h = area.height;
    let max_w = area.width;

    let height_units = max_h / BOARD_ROWS;
    let width_units = max_w / (BOARD_COLS * CELL_ASPECT_WIDTH);

    let h_unit = height_units.min(width_units);

    let final_h = h_unit * BOARD_ROWS * CELL_ASPECT_HEIGHT;
    let final_w = h_unit * BOARD_COLS * CELL_ASPECT_WIDTH;

    let center_y = area.y + (area.height - final_h) / 2;
    let center_x = area.x + (area.width - final_w) / 2;

    Rect {
        x: center_x,
        y: center_y,
        width: final_w,
        height: final_h,
    }
}

fn render_sidebar<B: Backend>(f: &mut Frame<B>, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(INFO_PANEL_HEIGHT),
            Constraint::Length(MOVES_PANEL_HEIGHT),
            Constraint::Min(0),
        ])
        .split(area);

    let mut text = String::new();

    match app.game_state {
        GameState::Ongoing => {
            text.push_str(format!("Turn: {:?}\n", app.current_turn).as_str());

            if app.board.is_in_check(app.current_turn) {
                text.push_str("CHECK!\n");
            }
        }
        GameState::Checkmate(color) => {
            let winner = match color {
                crate::pieces::Color::White => "Black",
                crate::pieces::Color::Black => "White",
            };
            text.push_str(format!("CHECKMATE!\n{} wins\n", winner).as_str());
        }
        GameState::Stalemate => {
            text.push_str("STALEMATE!\nDraw\n");
        }

    }
    let taken = app.taken.iter().fold(String::new(), |mut acc, m| {
        acc.push_str(&format!("{}\n", m.get_ui()));
        acc
    });
    f.render_widget(
        Paragraph::new(text).block(Block::default().title("Game Info").borders(Borders::ALL)),
        chunks[0],
    );
    f.render_widget(
        Block::default().title("Moves").borders(Borders::ALL),
        chunks[1],
    );
    f.render_widget(
        Paragraph::new(taken).block(Block::default().title("Captured").borders(Borders::ALL)),
        chunks[2],
    );
}
