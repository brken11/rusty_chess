pub enum Color {
    White,
    Black,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Piece {
    WhitePawn,
    BlackPawn,
    WhiteRook,
    BlackRook,
    WhiteKnight,
    BlackKnight,
    WhiteBishop,
    BlackBishop,
    WhiteQueen,
    BlackQueen,
    WhiteKing,
    BlackKing,
}

impl Piece {
    pub fn iter() -> impl Iterator<Item=Piece> {
        const PIECES: [Piece; 12] = [
            Piece::WhitePawn, Piece::BlackPawn,
            Piece::WhiteRook, Piece::BlackRook,
            Piece::WhiteKnight, Piece::BlackKnight,
            Piece::WhiteBishop, Piece::BlackBishop,
            Piece::WhiteQueen, Piece::BlackQueen,
            Piece::WhiteKing, Piece::BlackKing,
        ];
        PIECES.iter().copied()
    }

    pub fn to_index(&self) -> usize {
        *self as usize
    }

    pub fn from_index(index: usize) -> Option<Piece> {
        Piece::iter().nth(index)
    }

    pub fn to_char(&self) -> char {
        match self {
            Piece::WhitePawn => '♟', Piece::BlackPawn => '♙',
            Piece::WhiteRook => '♜', Piece::BlackRook => '♖',
            Piece::WhiteKnight => '♞', Piece::BlackKnight => '♘',
            Piece::WhiteBishop => '♝', Piece::BlackBishop => '♗',
            Piece::WhiteQueen => '♛', Piece::BlackQueen => '♕',
            Piece::WhiteKing => '♚', Piece::BlackKing => '♔',
        }
    }

}

impl Color {
    pub fn get_pawn(&self) -> Piece {
        match self {
            Color::White => Piece::WhitePawn,
            Color::Black => Piece::BlackPawn,
        }
    }
    pub fn get_rook(&self) -> Piece {
        match self {
            Color::White => Piece::WhiteRook,
            Color::Black => Piece::BlackRook,
        }
    }
    pub fn get_knight(&self) -> Piece {
        match self {
            Color::White => Piece::WhiteKnight,
            Color::Black => Piece::BlackKnight,
        }
    }
    pub fn get_bishop(&self) -> Piece {
        match self {
            Color::White => Piece::WhiteBishop,
            Color::Black => Piece::BlackBishop,
        }
    }
    pub fn get_queen(&self) -> Piece {
        match self {
            Color::White => Piece::WhiteQueen,
            Color::Black => Piece::BlackQueen,
        }
    }
    pub fn get_king(&self) -> Piece {
        match self {
            Color::White => Piece::WhiteKing,
            Color::Black => Piece::BlackKing,
        }
    }
}