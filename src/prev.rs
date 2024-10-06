use std::fmt;

const WHITE_PAWNS: [(usize, usize); 8] = [(6, 0), (6, 1), (6, 2), (6, 3), (6, 4), (6, 5), (6, 6), (6, 7)];
const BLACK_PAWNS: [(usize, usize); 8] = [(1, 0), (1, 1), (1, 2), (1, 3), (1, 4), (1, 5), (1, 6), (1, 7)];
const WHITE_HORSES: [(usize, usize); 2] = [(7, 1), (7, 6)];
const BLACK_HORSES: [(usize, usize); 2] = [(0, 1), (0, 6)];
const WHITE_BISHOPS: [(usize, usize); 2] = [(7, 2), (7, 5)];
const BLACK_BISHOPS: [(usize, usize); 2] = [(0, 2), (0, 5)];
const WHITE_ROOKS: [(usize, usize); 2] = [(7, 0), (7, 7)];
const BLACK_ROOKS: [(usize, usize); 2] = [(0, 0), (0, 7)];
const WHITE_QUEEN: (usize, usize) = (7, 3);
const BLACK_QUEEN: (usize, usize) = (0, 3);
const WHITE_KING: (usize, usize) = (7, 4);
const BLACK_KING: (usize, usize) = (0, 4);


#[derive(Copy, Clone, Debug)]
    enum Piece {
        Pawn(bool),
        Horse(bool),
        Rook(bool),
        Bishop(bool),
        Queen(bool),
        King(bool),
        None,
    }

    impl fmt::Display for Piece {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                Piece::Pawn(color) => {
                    match color {
                        true => write!(f, "{}", '\u{2659}'),
                        false => write!(f, "{}", '\u{2659}'),
                    }
                },
                Piece::Horse(color) => {
                    match color {
                        true => write!(f, "{}", '\u{2658}'),
                        false => write!(f, "{}", '\u{265E}'),
                    }
                },
                Piece::Rook(color) => {
                    match color {
                        true => write!(f, "{}", '\u{2656}'),
                        false => write!(f, "{}", '\u{265C}'),
                    }
                },
                Piece::Bishop(color) => {
                    match color {
                        true => write!(f, "{}", '\u{2657}'),
                        false => write!(f, "{}", '\u{265D}'),
                    }
                },
                Piece::Queen(color) => {
                    match color {
                        true => write!(f, "{}", '\u{2655}'),
                        false => write!(f, "{}", '\u{265B}'),
                    }
                },
                Piece::King(color) => {
                    match color {
                        true => write!(f, "{}", '\u{2654}'),
                        false => write!(f, "{}", '\u{265A}'),
                    }
                },
                Piece::None => write!(f, " "),
                // Piece::None => write!(f, "{}", '\u{25AD}'),
            }
        }
    }

    struct Chess {
        board: [[Piece; 8]; 8],
    }

    impl Chess {
        fn new() -> Self {
            let mut board = [[Piece::None; 8]; 8];

            for square in WHITE_PAWNS {
                board[square.0][square.1] = Piece::Pawn(true);
            }

            for square in BLACK_PAWNS {
                board[square.0][square.1] = Piece::Pawn(false);
            }

            for square in WHITE_HORSES {
                board[square.0][square.1] = Piece::Horse(true);
            }

            for square in BLACK_HORSES {
                board[square.0][square.1] = Piece::Horse(false);
            }

            for square in WHITE_BISHOPS {
                board[square.0][square.1] = Piece::Bishop(true);
            }

            for square in BLACK_BISHOPS {
                board[square.0][square.1] = Piece::Bishop(false);
            }

            for square in WHITE_ROOKS {
                board[square.0][square.1] = Piece::Rook(true);
            }

            for square in BLACK_ROOKS {
                board[square.0][square.1] = Piece::Rook(false);
            }

            board[WHITE_QUEEN.0][WHITE_QUEEN.1] = Piece::Queen(true);
            board[BLACK_QUEEN.0][BLACK_QUEEN.1] = Piece::Queen(false);
            board[WHITE_KING.0][WHITE_KING.1] = Piece::King(true);
            board[BLACK_KING.0][BLACK_KING.1] = Piece::King(false);

            Chess {
                board,
            } 
        }

        fn move_piece(&mut self, origin: (usize, usize)) {
            match self.board[origin.0][origin.1] {
                Piece::Pawn(color) => {
                    match color {
                        true => {
                            if self.board[origin.0+1][origin.1] == Piece::None() {
                                self.board[origin.0+1][origin.1] = self.board[origin.0][origin.1];
                                self.board[origin.0][origin.1] = Piece::None;
                            }
                        },

                        false => {

                        },
                    }
                    
                },
            }
            self.board[target.0][target.1] = self.board[origin.0][origin.1];
            self.board[origin.0][origin.1] = Piece::None;
        }
    } 

    impl fmt::Display for Chess {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            for row in self.board {
                for square in row {
                    write!(f,"{}", square)?
                }
                writeln!(f)?
            }
            Ok(())
        } 
    }

fn in_bounds(coords: (usize, usize)) -> bool {
    (coords.0 >= 0 && coords.0 <= 7) && (coords.0 >= 0 && coords.1 <= 7)
}

fn main() {
    let mut chess = Chess::new();


    println!("{}", chess);
    chess.move_piece((7, 0), (4, 0));
    println!("{}", chess);
}
