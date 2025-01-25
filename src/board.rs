pub(crate) mod pieces;

use std::fmt;
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
    const SQUARES: [&'static str; 64];
    fn get_row(&self) -> u8;
    fn get_rows(&self, ascending : bool) -> impl Iterator<Item = u8>;
    fn get_col(&self) -> u8;
    fn get_cols(&self, ascending : bool) -> impl Iterator<Item = u8>;
    fn get_pos_pair(&self) -> (u8, u8);
    fn get_index(&self) -> usize;
    fn get_file(&self) -> char;
    fn get_rank(&self) -> u8;
    fn new(row : u8, col : u8) -> Square;
    fn valid_new(row : u8, col : u8) -> Option<Square>;
    fn iter_files() -> impl Iterator<Item = char>;
    fn iter_ranks() -> impl Iterator<Item = u8>;
    fn iter_squares() -> impl Iterator<Item = Square>;
    fn to_square_str(&self) -> &str;
    fn to_square_string(&self) -> String;
}
impl SquareExt for Square {
    const SQUARES: [&'static str; 64] = [
        "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8",
        "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7",
        "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
        "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5",
        "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
        "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3",
        "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
        "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1",
    ];
    fn get_row(&self) -> u8 {
        self / 8
    }
    fn get_rows(&self, ascending : bool) -> impl Iterator<Item=u8> {
        if ascending {self.get_row() + 1..8} else {0..self.get_row()}
    }
    fn get_col(&self) -> u8 {
        self % 8
    }
    fn get_cols(&self, ascending : bool) -> impl Iterator<Item=u8> {
        if ascending {self.get_col() + 1..8} else {0..self.get_col()}
    }
    fn get_pos_pair(&self) -> (u8, u8) {
        (self.get_row(),self.get_col())
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
    fn new(row : u8, col : u8) -> Square {
        row * 8 + col
    }
    fn valid_new(row : u8, col : u8) -> Option<Square> {
        if row> 7 || col > 7 {return None} else {Some(Square::new(row, col))}
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
    fn to_square_str(&self) -> &str {
        Self::SQUARES[*self as usize]
    }
    fn to_square_string(&self) -> String {
        format!("{}{}", self.get_file(),self.get_rank())
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
        let bitboard_change = self.data[piece as usize] ^ bitboard;
        self.data[piece as usize] = bitboard;
        self.piece_locations ^= bitboard_change;
    }
    pub fn get_piece_count(&self) -> u8 {
        self.piece_locations.count_ones() as u8
    }

    pub fn get_piece_at(&self, square: Square) -> Option<Piece> {
        if self.piece_locations & (1 << square) == 0 {
            return None
        }
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
            Color::White => Some(self.data[Piece::WhiteKing as usize].trailing_zeros() as Square),
            Color::Black => Some(self.data[Piece::BlackKing as usize].trailing_zeros() as Square),
            _ => None
        }
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
        };
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

    pub fn sees_down_file(&self, square:Square, ascending: bool) -> Bitboard {
        let mut vision_board : Bitboard = 0;
        for s in square.get_rows(ascending) {
            vision_board |= 1 << s;
            if self.piece_locations & (1 << s) != 0 {
                break;
            }
        }
        vision_board
    }
    pub fn sees_down_rank(&self, square:Square, ascending: bool) -> Bitboard {
        let mut vision_board : Bitboard = 0;
        for s in square.get_cols(ascending) {
            vision_board |= 1 << s;
            if self.piece_locations & (1 << s) != 0 {
                break;
            };
        }
        vision_board
    }
    pub fn sees_down_diagonal(&self, square:Square, ascending_row: bool, ascending_col: bool) -> Bitboard {
        let mut vision_board : Bitboard = 0;
        let range = square.get_rows(ascending_row)
                                    .zip(square.get_cols(ascending_col));
        for s in range {
            let diagonal_square : Square = Square::new(s.0, s.1);
            vision_board |= (1 << diagonal_square);
            // println!("{}", diagonal_square.to_square_string());
            if self.piece_locations & (1 << diagonal_square) != 0 {
                break;
            }
        }
        vision_board
    }
    pub fn sees_like_knight(&self, square:Square) -> Bitboard {
        let mut vision_board : Bitboard = 0;
        let row = square.get_row();
        let col = square.get_col();
        if let Some(s) = Square::valid_new(row - 2, col - 1) {vision_board |= 1 << s;}
        if let Some(s) = Square::valid_new(row - 2, col + 1) {vision_board |= 1 << s;}
        if let Some(s) = Square::valid_new(row - 1, col - 2) {vision_board |= 1 << s;}
        if let Some(s) = Square::valid_new(row - 1, col + 2) {vision_board |= 1 << s;}
        if let Some(s) = Square::valid_new(row + 1, col - 2) {vision_board |= 1 << s;}
        if let Some(s) = Square::valid_new(row + 1, col + 2) {vision_board |= 1 << s;}
        if let Some(s) = Square::valid_new(row + 2, col - 1) {vision_board |= 1 << s;}
        if let Some(s) = Square::valid_new(row + 2, col + 1) {vision_board |= 1 << s;}
        vision_board
    }

    pub fn get_pieces_at_bitboard(&self, bitboard: Bitboard) -> Vec<(Piece, Square)> {
        let mut pieces : Vec<(Piece,Square)> = Vec::new();
        let mut bitboard = bitboard;
        while bitboard != 0 {
            let pos: u8 = bitboard.trailing_zeros() as u8;
            if let Some(piece) = self.get_piece_at(pos) {pieces.push((piece,pos));}
            bitboard &= (1<<pos);
        }
        pieces
    }
    pub fn get_active_pieces(&self) -> Vec<(Piece, Square)> {
        let mut pieces : Vec<(Piece,Square)> = Vec::new();
        for piece in Piece::iter_color_pieces(&self.active_player){
            pieces.extend(self.get_pieces_at_bitboard(self.data[piece.to_index()]));
        }
        pieces
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.to_string().fmt(f)
    }
}
// impl fmt::Display for Square {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         self.to_square_str().fmt(f)
//     }
// }