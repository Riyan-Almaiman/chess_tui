use tui::{
    Frame,
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;
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

const COLOR_LIGHT_SQUARE: Color = Color::Rgb(240, 217, 181);
const COLOR_DARK_SQUARE: Color = Color::Rgb(120, 70, 40);

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &App) {
    let size = f.size();
    let layout = main_layout(size);
    render_sidebar(f, layout.sidebar, app);
    draw_board(f, calculate_board_size(layout.board), app);
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
    let mut style = square_style(x, y);

    if Some((x, y)) == app.selected {
        style = style.bg(Color::Green);
    } else if x == app.cursor_x && y == app.cursor_y {
        style = style.bg(Color::Yellow);
    }

    f.render_widget(Block::default().style(style), area);
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

    let text = if let Some((x, y)) = app.selected {
        format!("Selected: ({}, {})", x, y)
    } else {
        "Selected: none".to_string()
    };

    f.render_widget(
        Paragraph::new(text).block(Block::default().title("Game Info").borders(Borders::ALL)),
        chunks[0],
    );
    f.render_widget(
        Block::default().title("Moves").borders(Borders::ALL),
        chunks[1],
    );
    f.render_widget(
        Block::default().title("Captured").borders(Borders::ALL),
        chunks[2],
    );
}
