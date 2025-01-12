pub(crate) mod pieces;

use pieces::Color;
use pieces::Piece;

pub enum BoardError {
    PieceNotFound,
    SquareOccupied,
}

pub struct CastlingRights {
    pub white_king_side: bool,
    pub white_queen_side: bool,
    pub black_king_side: bool,
    pub black_queen_side: bool,
}

pub type Bitboard = u64;
pub type Square = u8;
pub trait SquareExt {
    fn get_row(&self) -> u8;
    fn get_col(&self) -> u8;
    fn get_index(&self) -> usize;
    fn get_file(&self) -> char;
    fn get_rank(&self) -> u8;
    fn iter_files() -> impl Iterator<Item = char>;
    fn iter_ranks() -> impl Iterator<Item = u8>;
    fn iter_squares() -> impl Iterator<Item = Square>;
    fn to_square_string(&self) -> String;
}
impl SquareExt for Square {
    fn get_row(&self) -> u8 {
        self / 8
    }
    fn get_col(&self) -> u8 {
        self % 8
    }
    fn get_index(&self) -> usize {
        *self as usize
    }
    fn get_file(&self) -> char {
        match self % 8 {
             0 => 'a', 1 => 'b', 2 => 'c', 3 => 'd', 4 => 'e', 5 => 'f', 6 => 'g', 7 => 'h',
            _ => panic!("Invalid file index"),
        }
    }
    fn get_rank(&self) -> u8 {
         8 - self / 8
    }
    fn iter_files() -> impl Iterator<Item = char> {
        (0..8).map(|i| i as u8).map(|i| i.get_file())
    }
    fn iter_ranks() -> impl Iterator<Item = u8> {
        (0..8).map(|i| i as u8)
    }
    fn iter_squares() -> impl Iterator<Item = Square> {
        (0..64).map(|i| i as u8)
    }
    fn to_square_string(&self) -> String {
        format!("{}{}", self.get_rank(), self.get_file())
    }
}


pub struct Board {
    data: [Bitboard; 12],
    piece_locations: Bitboard,
    pub castling_rights: CastlingRights,
    pub en_passant_square: Option<Square>,
    pub half_move_clock: u8,
    pub full_move_number: u8,
    pub active_player: Color,
}

impl Board {
    pub fn std_new() -> Board {
        Board{
            castling_rights : CastlingRights{
                white_king_side : true,
                white_queen_side : true,
                black_king_side : true,
                black_queen_side : true,
            },
            en_passant_square : None,
            half_move_clock : 0,
            full_move_number : 1,
            active_player : Color::White,
            piece_locations : 0xFFFF00000000FFFF,
            data : [
                // Whites         // Blacks
                0x00FF000000000000,0x000000000000FF00, // Pawns
                0x8100000000000000,0x0000000000000081, // Rooks
                0x4200000000000000,0x0000000000000042, // Knights
                0x2400000000000000,0x0000000000000024, // Bishops
                0x0800000000000000,0x0000000000000008, // Queens
                0x1000000000000000,0x0000000000000010, // Kings
            ],
        }
    }
    pub fn empty_new() -> Board {
        Board{
            castling_rights : CastlingRights {
                white_king_side: false,
                white_queen_side: false,
                black_king_side: false,
                black_queen_side: false,
            },
            en_passant_square : None,
            half_move_clock : 0,
            full_move_number : 1,
            active_player : Color::White,
            piece_locations : 0,
            data : [0; 12],
        }
    }

    pub fn get_bitboard(&self, piece: Piece) -> Bitboard {
        self.data[piece as usize]
    }
    pub fn set_bitboard(&mut self, piece: Piece, bitboard: u64) {
        self.data[piece as usize] = bitboard;
    }
    pub fn get_piece_count(&self) -> u8 {
        self.piece_locations.count_ones() as u8
    }

    pub fn get_piece_at(&self, square: Square) -> Option<Piece> {
        for piece in pieces::Piece::iter() {
            if self.data[piece as usize] & (1 << square) != 0 {
                return Some(piece);
            }
        }
        None
    }
    pub fn find_piece_positions(&self, piece: Piece) -> Vec<Square> {
        let mut positions = Vec::new();
        let mut bitboard = self.data[piece as usize];
        while bitboard != 0{
            let pos = bitboard.trailing_zeros() as u8;
            positions.push(pos);
            bitboard &= !(1 << pos);
        }
        positions
    }
    pub fn king_square(&self) ->  Option<Square> {
        self.king_square_by_color(&self.active_player)
    }
    pub fn king_square_by_color(&self, color: &Color) -> Option<Square> {
        match color {
            Color::White => self.data[Piece::WhiteKing as usize].trailing_zeros(),
            Color::Black => self.data[Piece::BlackKing as usize].trailing_zeros()
        };
        None
    }

    pub fn remove_piece_at(&mut self, square: Square, piece: Piece) -> Result<(), BoardError>{
        if self.piece_locations & (1 << square) == 0 {
            return Err(BoardError::PieceNotFound);
        }
        self.data[piece as usize] &= !(1 << square);
        self.piece_locations &= !(1 << square);
        Ok(())
    }
    pub fn add_piece_at(&mut self, square: Square, piece: Piece) -> Result<(), BoardError>{
        if self.piece_locations & (1 << square) != 0 {
            return Err(BoardError::SquareOccupied);
        }
        self.data[piece as usize] |= 1 << square;
        self.piece_locations |= 1 << square;
        Ok(())
    }

    pub fn get_char_at(&self, square: Square) -> char {
        if let Some(piece) = self.get_piece_at(square) {
            return piece.to_char();
        }
        ' '
    }
    pub fn get_symbol_at(&self, square: Square) -> char {
        if let Some(piece) = self.get_piece_at(square) {
            return piece.to_symbol();
        }
        '-'
    }
    pub fn to_string(&self) -> String {
        let mut rendered_board = String::new();
        for row in Square::iter_ranks() {
            let square: Square = row * 8;// Compute the starting index for this rank
            rendered_board.push_str(&format!(
                " {} | {}  {}  {}  {}  {}  {}  {}  {} |\n",
                8 - row, //rank number
                self.get_symbol_at(square),
                self.get_symbol_at(square + 1),
                self.get_symbol_at(square + 2),
                self.get_symbol_at(square + 3),
                self.get_symbol_at(square + 4),
                self.get_symbol_at(square + 5),
                self.get_symbol_at(square + 6),
                self.get_symbol_at(square + 7)
            ));
        }
        rendered_board.push_str("   | a  b  c  d  e  f  g  h |\n");
        rendered_board
    }
}