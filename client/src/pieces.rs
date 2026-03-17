use crate::board::Board;

pub const WHITE_PAWN_UI: &str = "♙";
pub const WHITE_KNIGHT_UI: &str = "♘";
pub const WHITE_BISHOP_UI: &str = "♗";
pub const WHITE_ROOK_UI: &str = "♖";
pub const WHITE_QUEEN_UI: &str = "♕";
pub const WHITE_KING_UI: &str = "♔";

pub const BLACK_PAWN_UI: &str = "♟";
pub const BLACK_KNIGHT_UI: &str = "♞";
pub const BLACK_BISHOP_UI: &str = "♝";
pub const BLACK_ROOK_UI: &str = "♜";
pub const BLACK_QUEEN_UI: &str = "♛";
pub const BLACK_KING_UI: &str = "♚";
use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum Color {
    White,
    Black,
}

#[derive(Copy, Clone, PartialEq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Copy, Clone)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

impl Piece {
    pub fn get_ui(&self) -> &'static str {
        match (self.piece_type, self.color) {
            (PieceType::Pawn, Color::White) => WHITE_PAWN_UI,
            (PieceType::Pawn, Color::Black) => BLACK_PAWN_UI,
            (PieceType::Knight, Color::White) => WHITE_KNIGHT_UI,
            (PieceType::Knight, Color::Black) => BLACK_KNIGHT_UI,
            (PieceType::Bishop, Color::White) => WHITE_BISHOP_UI,
            (PieceType::Bishop, Color::Black) => BLACK_BISHOP_UI,
            (PieceType::Rook, Color::White) => WHITE_ROOK_UI,
            (PieceType::Rook, Color::Black) => BLACK_ROOK_UI,
            (PieceType::Queen, Color::White) => WHITE_QUEEN_UI,
            (PieceType::Queen, Color::Black) => BLACK_QUEEN_UI,
            (PieceType::King, Color::White) => WHITE_KING_UI,
            (PieceType::King, Color::Black) => BLACK_KING_UI,
        }
    }

    pub fn get_valid_moves(&self, pos: (usize, usize), board: &Board) -> Vec<(usize, usize)> {
        match self.piece_type {
            PieceType::Pawn => self.get_pawn_moves(pos, board),
            PieceType::Knight => self.get_knight_moves(pos, board),
            PieceType::Bishop => self.get_bishop_moves(pos, board),
            PieceType::Rook => self.get_rook_moves(pos, board),
            PieceType::Queen => self.get_queen_moves(pos, board),
            PieceType::King => self.get_king_moves(pos, board),
        }
    }
    pub fn get_attack_squares(&self, pos: (usize, usize), board: &Board) -> Vec<(usize, usize)> {
        match self.piece_type {
            PieceType::Pawn => {
                let mut attacks = Vec::new();
                let (r, c) = (pos.0 as i32, pos.1 as i32);
                let dir = if self.color == Color::White { -1 } else { 1 };

                for dc in [-1, 1] {
                    let nr = r + dir;
                    let nc = c + dc;

                    if nr >= 0 && nr < 8 && nc >= 0 && nc < 8 {
                        attacks.push((nr as usize, nc as usize));
                    }
                }

                attacks
            }

            PieceType::King => {
                let mut attacks = Vec::new();
                let (r, c) = (pos.0 as i32, pos.1 as i32);

                for dr in -1..=1 {
                    for dc in -1..=1 {
                        if dr == 0 && dc == 0 { continue; }

                        let nr = r + dr;
                        let nc = c + dc;

                        if nr >= 0 && nr < 8 && nc >= 0 && nc < 8 {
                            attacks.push((nr as usize, nc as usize));
                        }
                    }
                }

                attacks
            }

            PieceType::Knight => self.get_knight_moves(pos, board),

            PieceType::Bishop => self.get_line_attacks(pos, board, &[(1,1),(1,-1),(-1,1),(-1,-1)]),

            PieceType::Rook => self.get_line_attacks(pos, board, &[(1,0),(-1,0),(0,1),(0,-1)]),

            PieceType::Queen => self.get_line_attacks(
                pos,
                board,
                &[(1,0),(-1,0),(0,1),(0,-1),(1,1),(1,-1),(-1,1),(-1,-1)]
            ),
        }
    }fn get_line_attacks(
        &self,
        pos: (usize, usize),
        board: &Board,
        directions: &[(i32, i32)],
    ) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();
        let (r, c) = (pos.0 as i32, pos.1 as i32);

        for (dr, dc) in directions {
            let mut curr_r = r + dr;
            let mut curr_c = c + dc;

            while curr_r >= 0 && curr_r < 8 && curr_c >= 0 && curr_c < 8 {
                let target = (curr_r as usize, curr_c as usize);
                moves.push(target);

                if board.get_piece_at(target).is_some() {
                    break;
                }

                curr_r += dr;
                curr_c += dc;
            }
        }

        moves
    }
    fn get_line_moves(
        &self,
        pos: (usize, usize),
        board: &Board,
        directions: &[(i32, i32)],
    ) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();
        let (r, c) = (pos.0 as i32, pos.1 as i32);

        for (dr, dc) in directions {
            let mut curr_r = r + dr;
            let mut curr_c = c + dc;

            while curr_r >= 0 && curr_r < 8 && curr_c >= 0 && curr_c < 8 {
                let target = (curr_r as usize, curr_c as usize);

                match board.get_piece_at(target) {
                    None => {
                        moves.push(target); 
                    }
                    Some(other_piece) => {
                        if other_piece.color != self.color {
                            moves.push(target);
                        }
                        break; 
                    }
                }
                curr_r += dr;
                curr_c += dc;
            }
        }
        moves
    }
    fn get_pawn_moves(&self, pos: (usize, usize), board: &Board) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();
        let (r, c) = (pos.0 as i32, pos.1 as i32);
        let direction = if self.color == Color::White { -1 } else { 1 };

        let nr = r + direction;
        if nr >= 0 && nr < 8 && board.get_piece_at((nr as usize, c as usize)).is_none() {
            moves.push((nr as usize, c as usize));

            let start_row = if self.color == Color::White { 6 } else { 1 };
            let nr2 = r + (direction * 2);
            if r == start_row && board.get_piece_at((nr2 as usize, c as usize)).is_none() {
                moves.push((nr2 as usize, c as usize));
            }
        }

        for dc in [-1, 1] {
            let nc = c + dc;
            if nr >= 0 && nr < 8 && nc >= 0 && nc < 8 {
                if let Some(ep) = board.en_passant_target {
                    if ep == (nr as usize, nc as usize) {
                        let capture_row = if self.color == Color::White {
                            nr as usize + 1
                        } else {
                            nr as usize - 1
                        };

                        if let Some(other) = board.get_piece_at((capture_row, nc as usize)) {
                            if other.piece_type == PieceType::Pawn && other.color != self.color {
                                moves.push(ep);
                            }
                        }
                    }
                }

                if let Some(other) = board.get_piece_at((nr as usize, nc as usize)) {
                    if other.color != self.color {
                        moves.push((nr as usize, nc as usize));
                    }
                }
            }
        }
        moves
    }
    fn get_knight_moves(&self, pos: (usize, usize), board: &Board) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();
        let (r, c) = (pos.0 as i32, pos.1 as i32);
        let offsets = [
            (-2, -1),
            (-2, 1),
            (2, -1),
            (2, 1),
            (-1, -2),
            (-1, 2),
            (1, -2),
            (1, 2),
        ];

        for (dr, dc) in offsets {
            let nr = r + dr;
            let nc = c + dc;
            if nr >= 0 && nr < 8 && nc >= 0 && nc < 8 {
                let target = (nr as usize, nc as usize);
                match board.get_piece_at(target) {
                    None => moves.push(target),
                    Some(other) if other.color != self.color => moves.push(target),
                    _ => {} 
                }
            }
        }
        moves
    }

    fn get_rook_moves(&self, pos: (usize, usize), board: &Board) -> Vec<(usize, usize)> {
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        self.get_line_moves(pos, board, &directions)
    }

    fn get_bishop_moves(&self, pos: (usize, usize), board: &Board) -> Vec<(usize, usize)> {
        let directions = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
        self.get_line_moves(pos, board, &directions)
    }

    fn get_queen_moves(&self, pos: (usize, usize), board: &Board) -> Vec<(usize, usize)> {
        let directions = [
            (0, 1),
            (0, -1),
            (1, 0),
            (-1, 0), 
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1), 
        ];
        self.get_line_moves(pos, board, &directions)
    }

    fn get_king_moves(&self, pos: (usize, usize), board: &Board) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();
        let (r, c) = (pos.0 as i32, pos.1 as i32);

        for dr in -1..=1 {
            for dc in -1..=1 {
                if dr == 0 && dc == 0 {
                    continue;
                } // Can't move to same spot

                let nr = r + dr;
                let nc = c + dc;

                if nr >= 0 && nr < 8 && nc >= 0 && nc < 8 {
                    let target = (nr as usize, nc as usize);
                    match board.get_piece_at(target) {
                        None => moves.push(target),
                        Some(other) if other.color != self.color => moves.push(target),
                        _ => {}
                    }
                }
            }
        }
        if self.color == Color::White && !board.is_in_check(self.color) {
            if board.castle_white_kingside
                && board.get_piece_at((7, 5)).is_none()
                && board.get_piece_at((7, 6)).is_none()
                && matches!(
                    board.get_piece_at((7,7)),
                    Some(p) if p.piece_type == PieceType::Rook && p.color == Color::White
                )
            {
                let step1 = board.simulate_move((7, 4), (7, 5));
                if !step1.is_in_check(Color::White) {
                    let step2 = board.simulate_move((7, 4), (7, 6));
                    if !step2.is_in_check(Color::White) {
                        moves.push((7, 6));
                    }
                }
            }
            if board.castle_white_queenside
                && board.get_piece_at((7, 1)).is_none()
                && board.get_piece_at((7, 2)).is_none()
                && board.get_piece_at((7, 3)).is_none()
                && matches!(
                    board.get_piece_at((7,0)),
                    Some(p) if p.piece_type == PieceType::Rook && p.color == Color::White
                )
            {

                let step1 = board.simulate_move((7, 4), (7, 3));
                if !step1.is_in_check(Color::White) {
                    let step2 = board.simulate_move((7, 4), (7, 2));
                    if !step2.is_in_check(Color::White) {
                        moves.push((7, 2));
                    }
                }
            }
        }
        if self.color == Color::Black && !board.is_in_check(self.color) {
            if board.castle_black_kingside
                && board.get_piece_at((0, 5)).is_none()
                && board.get_piece_at((0, 6)).is_none()
                && matches!(
                    board.get_piece_at((0,7)),
                    Some(p) if p.piece_type == PieceType::Rook && p.color == Color::Black
                )
            {

                let step1 = board.simulate_move((0, 4), (0, 5));
                if !step1.is_in_check(Color::Black) {
                    let step2 = board.simulate_move((0, 4), (0,6));
                    if !step2.is_in_check(Color::Black) {
                        moves.push((0, 6));
                    }
                }
            }

            if board.castle_black_queenside
                && board.get_piece_at((0, 1)).is_none()
                && board.get_piece_at((0, 2)).is_none()
                && board.get_piece_at((0, 3)).is_none()
                && matches!(
                    board.get_piece_at((0,0)),
                    Some(p) if p.piece_type == PieceType::Rook && p.color == Color::Black
                )
            {

                let step1 = board.simulate_move((0, 4), (0, 3));
                if !step1.is_in_check(Color::Black) {
                    let step2 = board.simulate_move((0, 4), (0,2));
                    if !step2.is_in_check(Color::Black) {
                        moves.push((0, 2));
                    }
                }
            }
        }
        moves
    }
}
