use crate::board::Board;
use crate::pieces::{Color, Piece};
use crossterm::event::KeyCode;
#[derive(Clone)]
pub struct PieceMoves {
    pub position: (usize, usize),
    pub piece: Piece,
    pub moves: Vec<(usize, usize)>,
}
pub struct Player {
    pub color: Color,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameState {
    Ongoing,
    Checkmate(Color), // loser
    Stalemate,
}
pub struct App {
    pub board: Board,
    pub player_color: Color,
    pub cursor_x: usize,
    pub cursor_y: usize,
    pub selected: Option<(usize, usize)>,
    pub current_turn: Color,
    pub last_sent_move: Option<((usize, usize), (usize, usize))>,
    pub game_state: GameState,
    pub taken: Vec<Piece>,
    pub current_valid_moves: Vec<PieceMoves>,
}

impl App {
    pub fn new(player_color: Color) -> Self {
        Self {
            board: Board::new(),
            cursor_x: 0,
            cursor_y: 0,
            last_sent_move: None,
            selected: None,
            game_state: GameState::Ongoing,
            current_turn: Color::White,
            player_color,
            taken: Vec::new(),
            current_valid_moves: Vec::new(),
        }
    }
    pub fn update_game_state(&mut self) {
        let in_check = self.board.is_in_check(self.current_turn);

        let has_moves = !self.current_valid_moves.is_empty();

        self.game_state = if !has_moves {
            if in_check {
                GameState::Checkmate(self.current_turn)
            } else {
                GameState::Stalemate
            }
        } else {
            GameState::Ongoing
        };
    }  pub fn apply_network_move(&mut self, from: (usize, usize), to: (usize, usize)) {
        let taken = self.board.move_piece(from, to);
        if let Some(mut p) = self.board.get_piece_at(to) {
            if p.piece_type == crate::pieces::PieceType::Pawn {
                if (p.color == Color::White && to.0 == 0)
                    || (p.color == Color::Black && to.0 == 7)
                {
                    p.piece_type = crate::pieces::PieceType::Queen;
                    self.board.squares[to.0][to.1].piece = Some(p);
                }
            }
        }

        if let Some(piece) = taken {
            self.taken.push(piece);
        }

        self.current_turn = match self.current_turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };

        self.compute_all_moves();
        self.update_game_state();
    }
    pub fn handle_input(&mut self, key: KeyCode) -> bool {
        if key ==             KeyCode::Char('q') {
            return false;
        }

        if self.game_state != GameState::Ongoing {
            return true;
        }
        match key {
            KeyCode::Left => {
                if self.cursor_x > 0 {
                    self.cursor_x -= 1;
                }
            }
            KeyCode::Char(' ') | KeyCode::Enter => {
                self.handle_selection();
            }
            KeyCode::Right => {
                if self.cursor_x < 7 {
                    self.cursor_x += 1;
                }
            }
            KeyCode::Up => {
                if self.cursor_y > 0 {
                    self.cursor_y -= 1;
                }
            }
            KeyCode::Down => {
                if self.cursor_y < 7 {
                    self.cursor_y += 1;
                }
            }
            _ => (),
        }
        true
    }

    pub fn compute_all_moves(&mut self) {
        self.current_valid_moves.clear();

        for r in 0..8 {
            for c in 0..8 {
                if let Some(piece) = self.board.get_piece_at((r, c)) {
                    if piece.color == self.current_turn {
                        let moves = self.board.get_legal_moves((r, c));

                        if !moves.is_empty() {
                            self.current_valid_moves.push(PieceMoves {
                                position: (r, c),
                                piece,
                                moves,
                            });
                        }
                    }
                }
            }
        }
    }
    pub fn get_moves_for_pos(&self, pos: (usize, usize)) -> Option<&PieceMoves> {
        self.current_valid_moves.iter().find(|m| m.position == pos)
    }
    pub fn handle_selection(&mut self) -> Option<((usize, usize), (usize, usize))>{
        let current_pos = if self.player_color == Color::White {
            (self.cursor_y, self.cursor_x)
        } else {
            (7 - self.cursor_y, 7 - self.cursor_x)
        };
        if let Some(start_pos) = self.selected {
            if let Some(piece) = self.board.get_piece_at(current_pos) {
                if piece.color == self.current_turn && current_pos != start_pos {
                    self.selected = Some(current_pos);
                    return None;
                }
            }

            if let Some(pm) = self.get_moves_for_pos(start_pos) {
                if pm.moves.contains(&current_pos) {
                    let taken = self.board.move_piece(start_pos, current_pos);
                    let moved_piece = self.board.get_piece_at(current_pos);

                    if let Some(mut p) = moved_piece {
                        if p.piece_type == crate::pieces::PieceType::Pawn {
                            if (p.color == Color::White && current_pos.0 == 0)
                                || (p.color == Color::Black && current_pos.0 == 7)
                            {
                                p.piece_type = crate::pieces::PieceType::Queen;
                                self.board.squares[current_pos.0][current_pos.1].piece = Some(p);
                            }
                        }
                    }

                    if let Some(piece) = taken {
                        self.taken.push(piece);
                    }

                    self.current_turn = match self.current_turn {
                        Color::White => Color::Black,
                        Color::Black => Color::White,
                    };

                    self.selected = None;

                    self.compute_all_moves();
                    self.update_game_state();

                    return Some((start_pos, current_pos));

                }
            }

            self.selected = None;

        } else {
            if let Some(piece) = self.board.get_piece_at(current_pos) {
                if piece.color == self.current_turn {
                    self.selected = Some(current_pos);
                }
            }
        }None
    }
}
