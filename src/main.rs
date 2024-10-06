#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Piece {
    King(Color),
    Queen(Color),
    Rook(Color),
    Bishop(Color),
    Knight(Color),
    Pawn(Color),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Square {
    Empty,
    Occupied(Piece),
}

#[derive(Debug)]
struct Board {
    squares: [[Square; 8]; 8],
}

impl Board {
    fn new() -> Self {
        let mut board = Board {
            squares: [[Square::Empty; 8]; 8],
        };

        for i in 0..8 {
            board.squares[1][i] = Square::Occupied(Piece::Pawn(Color::Black));
            board.squares[6][i] = Square::Occupied(Piece::Pawn(Color::White));
        }

        let back_rank = [
            Piece::Rook(Color::Black),
            Piece::Knight(Color::Black),
            Piece::Bishop(Color::Black),
            Piece::Queen(Color::Black),
            Piece::King(Color::Black),
            Piece::Bishop(Color::Black),
            Piece::Knight(Color::Black),
            Piece::Rook(Color::Black),
        ];

        for (i, &piece) in back_rank.iter().enumerate() {
            board.squares[0][i] = Square::Occupied(piece);
            board.squares[7][i] = Square::Occupied(match piece {
                Piece::King(_) => Piece::King(Color::White),
                Piece::Queen(_) => Piece::Queen(Color::White),
                Piece::Rook(_) => Piece::Rook(Color::White),
                Piece::Bishop(_) => Piece::Bishop(Color::White),
                Piece::Knight(_) => Piece::Knight(Color::White),
                _ => piece, // No pawns here
            });
        }

        board
    }

    fn print(&self) {
        for row in self.squares.iter() {
            for square in row.iter() {
                match square {
                    Square::Empty => print!("- "),
                    Square::Occupied(piece) => match piece {
                        Piece::King(Color::White) => print!("K "),
                        Piece::Queen(Color::White) => print!("Q "),
                        Piece::Rook(Color::White) => print!("R "),
                        Piece::Bishop(Color::White) => print!("B "),
                        Piece::Knight(Color::White) => print!("N "),
                        Piece::Pawn(Color::White) => print!("P "),
                        Piece::King(Color::Black) => print!("k "),
                        Piece::Queen(Color::Black) => print!("q "),
                        Piece::Rook(Color::Black) => print!("r "),
                        Piece::Bishop(Color::Black) => print!("b "),
                        Piece::Knight(Color::Black) => print!("n "),
                        Piece::Pawn(Color::Black) => print!("p "),
                    },
                }
            }
            println!();
        }
    }
}

fn main() {
    let chess_board = Board::new();
    chess_board.print();
}
