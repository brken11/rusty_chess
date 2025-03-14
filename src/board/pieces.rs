use crate::board::Square;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    const PIECES: [Piece; 12] = [
        Piece::WhitePawn, Piece::BlackPawn,
        Piece::WhiteRook, Piece::BlackRook,
        Piece::WhiteKnight, Piece::BlackKnight,
        Piece::WhiteBishop, Piece::BlackBishop,
        Piece::WhiteQueen, Piece::BlackQueen,
        Piece::WhiteKing, Piece::BlackKing,
    ];
    const WHITE_PIECES: [Piece; 6] = [
        Piece::WhitePawn,
        Piece::WhiteRook,
        Piece::WhiteKnight,
        Piece::WhiteBishop,
        Piece::WhiteQueen,
        Piece::WhiteKing,
    ];
    const BLACK_PIECES: [Piece; 6] = [
        Piece::BlackPawn,
        Piece::BlackRook,
        Piece::BlackKnight,
        Piece::BlackBishop,
        Piece::BlackQueen,
        Piece::BlackKing,
    ];
    pub fn iter() -> impl Iterator<Item=Piece> {
        Piece::PIECES.iter().copied()
    }
    pub fn iter_white_pieces() -> impl Iterator<Item=Piece> {
        Piece::WHITE_PIECES.iter().copied()
    }
    pub fn iter_black_pieces() -> impl Iterator<Item=Piece> {
        Piece::BLACK_PIECES.iter().copied()
    }
    pub fn iter_color_pieces(color: &Color) -> impl Iterator<Item=Piece> {
        match color {
            Color::White => Piece::WHITE_PIECES.iter().copied(),
            Color::Black => Piece::BLACK_PIECES.iter().copied()
        }
    }

    pub fn get_color(&self) -> Color {
        match self {
            Piece::WhitePawn | Piece::WhiteRook | Piece::WhiteKnight | Piece::WhiteBishop | Piece::WhiteQueen | Piece::WhiteKing => Color::White,
            Piece::BlackPawn | Piece::BlackRook | Piece::BlackKnight | Piece::BlackBishop | Piece::BlackQueen | Piece::BlackKing => Color::Black,
        }
    }

    pub fn is_pawn(&self) -> bool {
        match self {
            Piece::WhitePawn | Piece::BlackPawn => true,
            _ => false,
        }
    }
    pub fn is_rook(&self) -> bool {
        match self {
            Piece::WhiteRook | Piece::BlackRook => true,
            _ => false,
        }
    }
    pub fn is_knight(&self) -> bool {
        match self {
            Piece::WhiteKnight | Piece::BlackKnight => true,
            _ => false,
        }
    }
    pub fn is_bishop(&self) -> bool {
        match self {
            Piece::WhiteBishop | Piece::BlackBishop => true,
            _ => false,
        }
    }
    pub fn is_queen(&self) -> bool {
        match self {
            Piece::WhiteQueen | Piece::BlackQueen => true,
            _ => false,
        }
    }
    pub fn is_king(&self) -> bool {
        match self {
            Piece::WhiteKing | Piece::BlackKing => true,
            _ => false,
        }
    }

    pub fn to_index(&self) -> usize {
        *self as usize
    }

    pub fn from_index(index: usize) -> Option<Piece> {
        Piece::iter().nth(index)
    }

    pub fn to_char(&self) -> char {
        match self {
            Piece::WhitePawn => 'P', Piece::BlackPawn => 'p',
            Piece::WhiteRook => 'R', Piece::BlackRook => 'r',
            Piece::WhiteKnight => 'N', Piece::BlackKnight => 'n',
            Piece::WhiteBishop => 'B', Piece::BlackBishop => 'b',
            Piece::WhiteQueen => 'Q', Piece::BlackQueen => 'q',
            Piece::WhiteKing => 'K', Piece::BlackKing => 'k',
        }
    }
    pub fn to_str(&self) -> Option<&str> {
        match self {
            Piece::WhiteRook => Some("R"), Piece::BlackRook => Some("r"),
            Piece::WhiteKnight => Some("N"), Piece::BlackKnight => Some("n"),
            Piece::WhiteBishop => Some("B"), Piece::BlackBishop => Some("b"),
            Piece::WhiteQueen => Some("Q"), Piece::BlackQueen => Some("q"),
            Piece::WhiteKing => Some("K"), Piece::BlackKing => Some("k"),
            _ => None,
        }
    }
    pub fn to_symbol(&self) -> char {
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
    pub fn toggle_color(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
    pub fn get_pawn(&self) -> Piece {
        match self {
            Color::White => Piece::WhitePawn,
            Color::Black => Piece::BlackPawn,
        }
    }
    pub fn get_pawn_direction(&self) -> u8 {
        /* Note, this is set to 8 as to make adding or 'subtracting' from base square to be
         * as simple as possible for pawn calculating the pawns destination square.
         */
        match self{
            Color::White => 248, // -8 for u8s
            Color::Black => 8,
        }
    }
    pub fn is_pawn_ascending(&self) -> bool {
        match self {
            Color::White => false,
            Color::Black => true,
        }
    }
    pub fn get_back_rank_row(&self) -> u8 {
        match self {
            Color::White => 7,
            Color::Black => 0,
        }
    }
    pub fn get_pawn_starting_row(&self) -> u8 {
        match self{
            Color::White => 6,
            Color::Black => 2,
        }
    }
    pub fn get_pawn_promotion_row(&self) -> u8 {
        match self{
            Color::White => 0,
            Color::Black => 7,
        }
    }
    pub fn get_promotion_pieces(&self) -> [Piece; 4] {
        match self {
            Color::White => [Piece::WhiteQueen, Piece::WhiteKnight, Piece::WhiteBishop, Piece::WhiteRook],
            Color::Black => [Piece::BlackQueen, Piece::BlackKnight, Piece::BlackBishop, Piece::BlackRook],
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