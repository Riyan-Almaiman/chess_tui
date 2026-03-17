use crate::pieces::{Color, Piece, PieceType};
#[derive(Clone)]
pub struct Square {
    pub piece: Option<Piece>,

}

#[derive(Clone)]
pub struct Board {
    pub squares: [[Square; 8]; 8],
    pub en_passant_target: Option<(usize, usize)>,

    pub castle_white_kingside: bool,
    pub castle_white_queenside: bool,
    pub castle_black_kingside: bool,
    pub castle_black_queenside: bool,
}
impl Board {
    pub fn new() -> Self {
        let mut squares = std::array::from_fn(|_| std::array::from_fn(|_| Square { piece: None }));

        // Setup Black pieces (Top of the array)
        squares[0] = Self::row_of_other_pieces(Color::Black);
        squares[1] = Self::row_of_pawns(Color::Black);

        // Setup White pieces (Bottom of the array)
        squares[6] = Self::row_of_pawns(Color::White);
        squares[7] = Self::row_of_other_pieces(Color::White);

        Board {
            squares,
            en_passant_target: None,
            castle_white_kingside: true,
            castle_white_queenside: true,
            castle_black_kingside: true,
            castle_black_queenside: true,
        }
    }
    pub fn find_king(&self, color: Color) -> Option<(usize, usize)> {
        for r in 0..8 {
            for c in 0..8 {
                if let Some(piece) = self.get_piece_at((r, c)) {
                    if piece.piece_type == PieceType::King && piece.color == color {
                        return Some((r, c));
                    }
                }
            }
        }
        None
    }
    pub fn is_in_check(&self, color: Color) -> bool {
        let king_pos = match self.find_king(color) {
            Some(pos) => pos,
            None => return false,
        };

        for r in 0..8 {
            for c in 0..8 {
                if let Some(piece) = self.get_piece_at((r, c)) {
                    if piece.color != color {

                        let attacks = piece.get_attack_squares((r, c), self);
                        if attacks.contains(&king_pos) {
                            return true;
                        }
                        }

                }
            }
        }

        false
    }
    pub fn get_legal_moves(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let piece = match self.get_piece_at(pos) {
            Some(p) => p,
            None => return vec![],
        };

        let pseudo_moves = piece.get_valid_moves(pos, self);

        pseudo_moves
            .into_iter()
            .filter(|&target| {
                let simulated = self.simulate_move(pos, target);
                !simulated.is_in_check(piece.color)
            })
            .collect()
    }
    pub fn simulate_move(&self, from: (usize, usize), to: (usize, usize)) -> Board {
        let mut new_board = self.clone();
        new_board.move_piece(from, to);
        new_board
    }
    pub fn move_piece(&mut self, from: (usize, usize), to: (usize, usize)) -> Option<Piece> {
        let mut taken = self.squares[to.0][to.1].piece.take();
        let piece = self.squares[from.0][from.1].piece.take();

        let old_ep = self.en_passant_target;
        self.en_passant_target = None;

        if let Some(p) = piece {
            match p.piece_type {
                PieceType::Pawn => {
                    if let Some(ep) = old_ep {
                        if to == ep && taken.is_none() {
                            let capture_row = if p.color == Color::White {
                                to.0 + 1
                            } else {
                                to.0 - 1
                            };

                            taken = self.squares[capture_row][to.1].piece.take();
                        }
                    }

                    if (from.0 as i32 - to.0 as i32).abs() == 2 {
                        let mid_row = (from.0 + to.0) / 2;
                        self.en_passant_target = Some((mid_row, from.1));
                    }
                }
                PieceType::Rook => {
                    if from == (7, 0) {
                        self.castle_white_queenside = false;
                    }
                    if from == (7, 7) {
                        self.castle_white_kingside = false;
                    }
                    if from == (0, 0) {
                        self.castle_black_queenside = false;
                    }
                    if from == (0, 7) {
                        self.castle_black_kingside = false;
                    }
                }
                PieceType::King => {
                    if (from.1 as i32 - to.1 as i32).abs() == 2 {
                        // kingside
                        if to.1 == 6 {
                            let rook = self.squares[from.0][7].piece.take();
                            self.squares[from.0][5].piece = rook;
                        }

                        // queenside
                        if to.1 == 2 {
                            let rook = self.squares[from.0][0].piece.take();
                            self.squares[from.0][3].piece = rook;
                        }
                    }

                    match p.color {
                        Color::White => {
                            self.castle_white_kingside = false;
                            self.castle_white_queenside = false;
                        }
                        Color::Black => {
                            self.castle_black_kingside = false;
                            self.castle_black_queenside = false;
                        }
                    }
                }
                _ => (),
            }

            self.squares[to.0][to.1].piece = Some(p);
        }
        if let Some(captured) = taken {
            if captured.piece_type == PieceType::Rook {
                if to == (7, 0) {
                    self.castle_white_queenside = false;
                }
                if to == (7, 7) {
                    self.castle_white_kingside = false;
                }

                if to == (0, 0) {
                    self.castle_black_queenside = false;
                }
                if to == (0, 7) {
                    self.castle_black_kingside = false;
                }
            }
        }
        taken
    }
    pub fn get_piece_at(&self, pos: (usize, usize)) -> Option<Piece> {
        self.squares[pos.0][pos.1].piece
    }

    fn row_of_pawns(color: Color) -> [Square; 8] {
        std::array::from_fn(|_| Square {
            piece: Some(Piece {
                piece_type: PieceType::Pawn,
                color,
            }),
        })
    }

    fn row_of_other_pieces(color: Color) -> [Square; 8] {
        let types = [
            PieceType::Rook,
            PieceType::Knight,
            PieceType::Bishop,
            PieceType::Queen,
            PieceType::King,
            PieceType::Bishop,
            PieceType::Knight,
            PieceType::Rook,
        ];
        let mut row = std::array::from_fn(|_| Square { piece: None });
        for i in 0..8 {
            row[i].piece = Some(Piece {
                piece_type: types[i],
                color,
            });
        }
        row
    }
}
