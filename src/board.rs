//! Chess board module.
//!
//! This module defines the board, castling rights, bitboards, and extended square functionality.
//! It provides methods to query and update board state, move pieces, and render the board.
pub(crate) mod pieces;

use std::fmt;
use pieces::Color;
use pieces::Piece;
use itertools::Either;

/// Errors that may occur when performing board operations.
pub enum BoardError {
    /// The requested piece could not be found at the given square.
    PieceNotFound,
    /// The target square is already occupied.
    SquareOccupied,
}

/// Contains castling rights for both white and black.
pub struct CastlingRights {
    /// White can castle king-side.
    pub white_king_side: bool,
    /// White can castle queen-side.
    pub white_queen_side: bool,
    /// Black can castle king-side.
    pub black_king_side: bool,
    /// Black can castle queen-side.
    pub black_queen_side: bool,
}

/// A bitboard is used to represent piece positions using a 64-bit number.
pub type Bitboard = u64;
/// A square on the chessboard represented as a value between 0 and 63.
pub type Square = u8;
/// Provides extended functionality for chess board squares.
pub trait SquareExt {
    /// Array of square labels, from "a8" to "h1".
    const SQUARES: [&'static str; 64];
    /// Returns the row (0-based) for the square.
    fn get_row(&self) -> u8;
    /// Returns an iterator over rows relative to this square.
    ///
    /// If `ascending` is true, the iterator starts at the next row up to the top (row 7).
    /// Otherwise, it iterates from row 0 up to the current row (exclusive).
    fn get_rows(&self, ascending : bool) -> Box<dyn Iterator<Item=u8>>;
    /// Returns an iterator over squares in the same column (file) in the given direction.
    fn get_row_squares(&self, ascending : bool) -> impl Iterator<Item = Square>;
    /// Returns the column (0-based) for the square.
    fn get_col(&self) -> u8;
    /// Returns an iterator over columns relative to this square.
    fn get_cols(&self, ascending : bool) -> Box<dyn Iterator<Item=u8>>;
    /// Returns an iterator over squares in the same row (rank) in the given direction.
    fn get_col_squares(&self, ascending : bool) -> impl Iterator<Item = Square>;
    /// Returns a tuple of (row, column) for the square.
    fn get_pos_pair(&self) -> (u8, u8);
    /// Returns the index of the square as a `usize`.
    fn get_index(&self) -> usize;
    /// Returns the file (column letter, e.g., 'a' through 'h') for the square.
    fn get_file(&self) -> char;
    /// Returns the rank (1-based, e.g., 1 through 8) for the square.
    fn get_rank(&self) -> u8;
    /// Creates a new square from the given row and column.
    fn new(row : u8, col : u8) -> Square;
    /// Creates a new square from the given row and column if they are valid, otherwise returns `None`.
    fn valid_new(row : u8, col : u8) -> Option<Square>;
    /// Returns an iterator over the file letters.
    fn iter_files() -> impl Iterator<Item = char>;
    /// Returns an iterator over the rank numbers.
    fn iter_ranks() -> impl Iterator<Item = u8>;
    /// Returns an iterator over all squares (0 to 63).
    fn iter_squares() -> impl Iterator<Item = Square>;
    /// Returns the string slice representation of the square (e.g., "e4").
    fn to_square_str(&self) -> &str;
    /// Returns the string representation of the square.
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
    fn get_rows(&self, ascending : bool) -> Box<dyn Iterator<Item=u8>> {
        if ascending {
            Box::new((self.get_row() + 1..8).rev())
        } else{
            Box::new(0..self.get_row())
        }
    }
    fn get_row_squares(&self, ascending : bool) -> impl Iterator<Item = Square> {
        if ascending {
            Either::Left((self+8..64).step_by(8))
        } else {
            Either::Right((self.get_col()..*self).step_by(8).rev())
        }
    }
    fn get_col(&self) -> u8 {
        self % 8
    }
    fn get_cols(&self, ascending : bool) -> Box<dyn Iterator<Item=u8>> {
        if ascending {
            Box::new(self.get_col() + 1..8)
        } else {
            Box::new((0..self.get_col()).into_iter().rev())
        }
    }
    fn get_col_squares(&self, ascending : bool) -> impl Iterator<Item = Square> {
        if ascending {
            Either::Left(self+1..(self+8-self%8))
        } else {
            Either::Right((self-self%8..*self).rev())
        }
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

/// Represents a chess board with pieces, castling rights, en passant state,
/// move counters, and the active player.
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
    /// Creates a board with the standard starting position.
    ///
    /// # Returns
    ///
    /// A `Board` with pieces arranged in their initial positions, castling rights enabled,
    /// and the active player set to white.
    ///
    /// # Example
    ///
    /// ```rust
    /// let board = Board::std_new();
    /// assert_eq!(board.active_player, Color::White);
    /// ```
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
    /// Creates an empty board with no pieces.
    ///
    /// # Returns
    ///
    /// A `Board` with no pieces, no castling rights, and white as the active player.
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
    pub fn clone(&self) -> Board {
        Board{
            castling_rights : CastlingRights{
                white_king_side : self.castling_rights.white_king_side,
                white_queen_side : self.castling_rights.white_queen_side,
                black_king_side : self.castling_rights.black_king_side,
                black_queen_side : self.castling_rights.black_queen_side
            },
            en_passant_square : self.en_passant_square,
            half_move_clock : self.half_move_clock,
            full_move_number : self.full_move_number,
            active_player : self.active_player,
            piece_locations : self.piece_locations,
            data : self.data
        }
    }

    /// Retrieves the bitboard corresponding to a given piece.
    ///
    /// # Arguments
    ///
    /// * `piece` - The piece whose bitboard is requested.
    ///
    /// # Returns
    ///
    /// A `Bitboard` representing the positions of the specified piece.
    pub fn get_bitboard(&self, piece: Piece) -> Bitboard {
        self.data[piece as usize]
    }
    /// Sets the bitboard for a given piece and updates overall piece locations.
    ///
    /// # Arguments
    ///
    /// * `piece` - The piece to update.
    /// * `bitboard` - The new bitboard for the piece.
    pub fn set_bitboard(&mut self, piece: Piece, bitboard: u64) {
        let bitboard_change = self.data[piece as usize] ^ bitboard;
        self.data[piece as usize] = bitboard;
        self.piece_locations ^= bitboard_change;
    }
    /// Returns the total number of pieces on the board.
    ///
    /// # Returns
    ///
    /// The piece count as a `u8`.
    pub fn get_piece_count(&self) -> u8 {
        self.piece_locations.count_ones() as u8
    }

    /// Retrieves the piece at a given square.
    ///
    /// # Arguments
    ///
    /// * `square` - The square to query.
    ///
    /// # Returns
    ///
    /// `Some(Piece)` if a piece exists at the square, otherwise `None`.
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
    /// Checks if a piece exists at the specified square.
    ///
    /// # Arguments
    ///
    /// * `square` - The square to check.
    ///
    /// # Returns
    ///
    /// `true` if a piece is present, otherwise `false`.
    pub fn is_piece_at(&self, square: Square) -> bool {
        self.piece_locations & (1 << square) != 0
    }
    /// Retrieves the piece at the specified square if it matches the given color.
    ///
    /// # Arguments
    ///
    /// * `square` - The square to query.
    /// * `color` - The color of the piece to look for.
    ///
    /// # Returns
    ///
    /// `Some(Piece)` if a matching piece is found, otherwise `None`.
    pub fn get_colored_piece_at(&self, square: Square, color: Color) -> Option<Piece> {
        match color {
            Color::White => {
                for piece in Piece::iter_white_pieces(){
                    if self.data[piece as usize] & (1 << square) != 0 {
                        return Some(piece)
                    }
                }
                None
            },
            Color::Black => {
                for piece in Piece::iter_black_pieces(){
                    if self.data[piece as usize] & (1 << square) != 0 {
                        return Some(piece)
                    }
                }
                None
            }
        }
    }
    /// Finds all positions where the specified piece is located.
    ///
    /// # Arguments
    ///
    /// * `piece` - The piece to search for.
    ///
    /// # Returns
    ///
    /// A vector of squares where the piece is found.
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
    /// Retrieves the king's square for the active player.
    ///
    /// # Returns
    ///
    /// `Some(Square)` if the king is found, otherwise `None`.
    pub fn king_square(&self) ->  Option<Square> {
        self.king_square_by_color(&self.active_player)
    }
    /// Retrieves the king's square for the specified color.
    ///
    /// # Arguments
    ///
    /// * `color` - The color of the king.
    ///
    /// # Returns
    ///
    /// `Some(Square)` if found, otherwise `None`.
    pub fn king_square_by_color(&self, color: &Color) -> Option<Square> {
        match color {
            Color::White => Some(self.data[Piece::WhiteKing as usize].trailing_zeros() as Square),
            Color::Black => Some(self.data[Piece::BlackKing as usize].trailing_zeros() as Square)
        }
    }

    /// Removes a piece from a specified square.
    ///
    /// # Arguments
    ///
    /// * `square` - The square from which to remove the piece.
    /// * `piece` - The piece to remove.
    ///
    /// # Returns
    ///
    /// `Ok(())` if removal succeeds, otherwise a `BoardError`.
    pub fn remove_piece_at(&mut self, square: Square, piece: Piece) -> Result<(), BoardError>{
        if self.piece_locations & (1 << square) == 0 {
            return Err(BoardError::PieceNotFound);
        }
        self.data[piece as usize] &= !(1 << square);
        self.piece_locations &= !(1 << square);
        Ok(())
    }
    /// Adds a piece to the specified square.
    ///
    /// # Arguments
    ///
    /// * `square` - The square at which to add the piece.
    /// * `piece` - The piece to add.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the piece is added successfully, otherwise a `BoardError`.
    pub fn add_piece_at(&mut self, square: Square, piece: Piece) -> Result<(), BoardError>{
        if self.piece_locations & (1 << square) != 0 {
            return Err(BoardError::SquareOccupied);
        }
        self.data[piece as usize] |= 1 << square;
        self.piece_locations |= 1 << square;
        Ok(())
    }

    /// Retrieves the character representation of the piece at the given square.
    ///
    /// # Arguments
    ///
    /// * `square` - The square to query.
    ///
    /// # Returns
    ///
    /// The character representing the piece, or a space if none exists.
    pub fn get_char_at(&self, square: Square) -> char {
        if let Some(piece) = self.get_piece_at(square) {
            return piece.to_char();
        };
        ' '
    }
    /// Retrieves the symbol representation of the piece at the given square.
    ///
    /// # Arguments
    ///
    /// * `square` - The square to query.
    ///
    /// # Returns
    ///
    /// The symbol representing the piece, or '-' if none exists.
    pub fn get_symbol_at(&self, square: Square) -> char {
        if let Some(piece) = self.get_piece_at(square) {
            return piece.to_symbol();
        }
        '-'
    }
    /// Renders the board as a formatted string.
    ///
    /// # Returns
    ///
    /// A string representing the board layout.
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

    /// Computes clear squares along a file starting from a given square.
    ///
    /// Iterates through squares along the file in the specified direction until a piece is encountered.
    ///
    /// # Arguments
    ///
    /// * `square` - The starting square.
    /// * `ascending` - If true, iterates upward; otherwise, downward.
    ///
    /// # Returns
    ///
    /// A bitboard representing clear squares along the file.
    pub fn sees_down_file(&self, square:Square, ascending: bool) -> Bitboard {
        let mut vision_board : Bitboard = 0;
        for s in square.get_row_squares(ascending) {
            vision_board |= 1 << s;
            if self.piece_locations & (1 << s) != 0 {
                break;
            }
        }
        vision_board
    }
    /// Computes clear squares along a rank starting from a given square.
    ///
    /// Iterates through squares along the rank in the specified direction until a piece is encountered.
    ///
    /// # Arguments
    ///
    /// * `square` - The starting square.
    /// * `ascending` - If true, iterates rightward; otherwise, leftward.
    ///
    /// # Returns
    ///
    /// A bitboard representing clear squares along the rank.
    pub fn sees_down_rank(&self, square:Square, ascending: bool) -> Bitboard {
        let mut vision_board : Bitboard = 0;
        for s in square.get_col_squares(ascending) {
            vision_board |= 1 << s;
            if self.piece_locations & (1 << s) != 0 {
                break;
            };
        }
        vision_board
    }
    /// Computes clear squares along a diagonal starting from a given square.
    ///
    /// Iterates along the diagonal defined by `ascending_row` and `ascending_col` until a piece is encountered.
    ///
    /// # Arguments
    ///
    /// * `square` - The starting square.
    /// * `ascending_row` - If true, rows are iterated in ascending order; otherwise, descending.
    /// * `ascending_col` - If true, columns are iterated in ascending order; otherwise, descending.
    ///
    /// # Returns
    ///
    /// A bitboard representing clear squares along the diagonal.
    pub fn sees_down_diagonal(&self, square:Square, ascending_row: bool, ascending_col: bool) -> Bitboard {
        let mut vision_board : Bitboard = 0;
        let range = square.get_rows(ascending_row)
                                    .zip(square.get_cols(ascending_col));
        for s in range {
            let diagonal_square : Square = Square::new(s.0, s.1);
            vision_board |= 1 << diagonal_square;
            // println!("{}", diagonal_square.to_square_string());
            if self.piece_locations & (1 << diagonal_square) != 0 {
                break;
            }
        }
        vision_board
    }
    /// Computes the squares a knight can reach from the given square.
    ///
    /// # Arguments
    ///
    /// * `square` - The starting square.
    ///
    /// # Returns
    ///
    /// A bitboard representing the knight's move possibilities.
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

    /// Computes clear moves along a file and returns potential capture information.
    ///
    /// Iterates along the file until a piece is encountered. If a piece of the given color is found,
    /// returns that square as capturable.
    ///
    /// # Arguments
    ///
    /// * `square` - The starting square.
    /// * `ascending` - If true, iterates upward; otherwise, downward.
    /// * `color` - The color to check for a capturable piece.
    ///
    /// # Returns
    ///
    /// A tuple containing:
    /// - A bitboard of clear squares.
    /// - An optional square where a capturable piece is found.
    pub fn clear_n_capture_down_file(&self, square:Square, ascending: bool, color: Color) -> (Bitboard, Option<Square>) {
        //Returns a bitboard of clear moves (without capture) and an Option of Square if a capturable piece is present.
        let mut vision_board : Bitboard = 0;
        for s in square.get_row_squares(ascending) {
            if self.piece_locations & (1 << s) != 0 {
                return match self.get_colored_piece_at(s, color) {
                    Some(_) => (vision_board, Some(s)),
                    None => (vision_board, None),
                }
            }
            vision_board |= 1 << s;
        }
        (vision_board, None)
    }
    /// Computes clear moves along a rank and returns potential capture information.
    ///
    /// Iterates along the rank until a piece is encountered. If a piece of the given color is found,
    /// returns that square as capturable.
    ///
    /// # Arguments
    ///
    /// * `square` - The starting square.
    /// * `ascending` - If true, iterates rightward; otherwise, leftward.
    /// * `color` - The color to check for a capturable piece.
    ///
    /// # Returns
    ///
    /// A tuple containing:
    /// - A bitboard of clear squares.
    /// - An optional square where a capturable piece is found.
    pub fn clear_n_capture_down_rank(&self, square:Square, ascending: bool, color: Color) -> (Bitboard, Option<Square>) {
        let mut vision_board : Bitboard = 0;
        for s in square.get_col_squares(ascending) {
            if self.piece_locations & (1 << s) != 0 {
                return match self.get_colored_piece_at(s, color) {
                    Some(_) => (vision_board, Some(s)),
                    None => (vision_board, None),
                }
            }
            vision_board |= 1 << s;
        }
        (vision_board, None)
    }
    /// Computes clear moves along a diagonal and returns potential capture information.
    ///
    /// Iterates along the diagonal defined by the given row and column directions until a piece is encountered.
    /// If a piece of the specified color is encountered, that square is returned as capturable.
    ///
    /// # Arguments
    ///
    /// * `square` - The starting square.
    /// * `ascending_row` - If true, rows are iterated in ascending order; otherwise, descending.
    /// * `ascending_col` - If true, columns are iterated in ascending order; otherwise, descending.
    /// * `color` - The color to check for a capturable piece.
    ///
    /// # Returns
    ///
    /// A tuple containing:
    /// - A bitboard of clear squares along the diagonal.
    /// - An optional square where a capturable piece is found.
    pub fn clear_n_capture_down_diagonal(&self, square:Square, ascending_row: bool, ascending_col: bool, color: Color) -> (Bitboard, Option<Square>) {
        let mut vision_board : Bitboard = 0;
        let range = square.get_rows(ascending_row)
            .zip(square.get_cols(ascending_col));
        for s in range {
            let diagonal_square : Square = Square::new(s.0, s.1);
            // println!("{}", diagonal_square.to_square_string());
            if self.piece_locations & (1 << diagonal_square) != 0 {
                return match self.get_colored_piece_at(diagonal_square, color) {
                    Some(_) => (vision_board, Some(diagonal_square)),
                    None => (vision_board, None),
                }
            }
            vision_board |= 1 << diagonal_square;
        }
        (vision_board, None)
    }

    /// Retrieves all pieces present on the provided bitboard.
    ///
    /// # Arguments
    ///
    /// * `bitboard` - The bitboard to inspect.
    ///
    /// # Returns
    ///
    /// A vector of `(Piece, Square)` tuples for each piece found.
    pub fn get_pieces_at_bitboard(&self, bitboard: Bitboard) -> Vec<(Piece, Square)> {
        let mut pieces : Vec<(Piece,Square)> = Vec::new();
        let mut bitboard = bitboard;
        while bitboard != 0 {
            let pos: u8 = bitboard.trailing_zeros() as u8;
            if let Some(piece) = self.get_piece_at(pos) {pieces.push((piece,pos));}
            bitboard &= 1<<pos;
        }
        pieces
    }
    /// Retrieves all active pieces for the active player along with their positions.
    ///
    /// # Returns
    ///
    /// A vector of `(Piece, Square)` tuples for each active piece.
    pub fn get_active_pieces(&self) -> Vec<(Piece, Square)> {
        let mut pieces : Vec<(Piece,Square)> = Vec::new();
        for piece in Piece::iter_color_pieces(&self.active_player){
            pieces.extend(self.get_pieces_at_bitboard(self.data[piece.to_index()]));
        }
        pieces
    }
}

impl fmt::Display for Board {
    /// Formats the board for display.
    ///
    /// This implementation renders the board using `to_string()`.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.to_string().fmt(f)
    }
}

// If you need to implement display for squares, you can uncomment and complete this:
//
// impl fmt::Display for Square {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         self.to_square_str().fmt(f)
//     }
// }