//! Chess board module.
//!
//! This module defines the board, castling rights, bitboards, and extended square functionality.
//! It provides methods to query and update board state, add/remove pieces, and render the board.
pub mod pieces;
pub mod square;

pub(crate) use pieces::Color;
pub use pieces::Piece;
pub(crate) use square::Square as Square;
pub(crate) use square::SquareExt;
pub(crate) use square::square_arithmetic;
use crate::board::square::Col;
use crate::board::square::Row;

/// Errors that may occur when performing board operations.
pub enum BoardError {
    /// The requested piece could not be found at the given square.
    PieceNotFound,
    /// The target square is already occupied.
    SquareOccupied,
}

/// Contains castling rights for both white and black.
pub type CastlingRights = u8;
pub trait CastlingRightsExt{
    const NONE_CAN_CASTLE: CastlingRights;
    const ALL_CAN_CASTLE: CastlingRights;
    const KINGSIDE_MASK: CastlingRights;
    const QUEENSIDE_MASK: CastlingRights;
    const WHITE_KING_MASK: CastlingRights;
    const BLACK_KING_MASK: CastlingRights;
    fn can_castle(self, color: Color, kingside: bool) -> bool;
    fn can_castle_white_king_side(self) -> bool;
    fn can_castle_white_queen_side(self) -> bool;
    fn can_castle_black_king_side(self) -> bool;
    fn can_castle_black_queen_side(self) -> bool;
    fn king_moved(&mut self, king_color: Color);
    fn rook_moved(&mut self, rook_color: Color, kingside: bool);
}
impl CastlingRightsExt for CastlingRights {
    const NONE_CAN_CASTLE: CastlingRights = 0;
    const ALL_CAN_CASTLE: CastlingRights = 0b0000_1111;
    const KINGSIDE_MASK: CastlingRights = 0b0000_0101;
    const QUEENSIDE_MASK: CastlingRights = 0b0000_1010;
    const WHITE_KING_MASK: CastlingRights = 0b0000_0011;
    const BLACK_KING_MASK: CastlingRights = 0b0000_1100;
    #[inline]
    fn can_castle(self, color: Color, kingside: bool) -> bool {
        let mask = match kingside{
            true => CastlingRights::KINGSIDE_MASK,
            false => CastlingRights::QUEENSIDE_MASK,
        };
        match color{
            Color::White => (self & mask & CastlingRights::WHITE_KING_MASK) != 0,
            Color::Black => (self & mask & CastlingRights::BLACK_KING_MASK) != 0
        }
    }
    #[inline]
    fn can_castle_white_king_side(self) -> bool {
        self & Self::WHITE_KING_MASK & Self::KINGSIDE_MASK > 0
    }
    #[inline]
    fn can_castle_white_queen_side(self) -> bool {
        self & Self::WHITE_KING_MASK & Self::QUEENSIDE_MASK > 0
    }
    #[inline]
    fn can_castle_black_king_side(self) -> bool {
        self & Self::BLACK_KING_MASK & Self::KINGSIDE_MASK > 0
    }
    #[inline]
    fn can_castle_black_queen_side(self) -> bool {
        self & Self::BLACK_KING_MASK & Self::QUEENSIDE_MASK > 0
    }
    #[inline]
    fn king_moved(&mut self, king_color: Color){
        match king_color{
            Color::White => *self &= ! CastlingRights::WHITE_KING_MASK,
            Color::Black => *self &= ! CastlingRights::BLACK_KING_MASK
        };
    }
    #[inline]
    fn rook_moved(&mut self, rook_color: Color, kingside: bool){
        let mask = match kingside{
            true => CastlingRights::KINGSIDE_MASK,
            false => CastlingRights::QUEENSIDE_MASK,
        };
        match rook_color{
            Color::White => *self &= ! (mask & CastlingRights::WHITE_KING_MASK),
            Color::Black => *self &= ! (mask & CastlingRights::BLACK_KING_MASK)
        }
    }
}


/// A bitboard is used to represent piece positions using a 64-bit number.
pub type Bitboard = u64;
pub trait BitboardExt{
    fn get_bitboard_from_row(row: Row) -> Bitboard;
    fn get_bitboard_from_col(col: Col) -> Bitboard;
}
impl BitboardExt for Bitboard {
    fn get_bitboard_from_row(row: Row) -> Bitboard {
        0x00000000000000FF << (row * Square::COLS)
    }
    fn get_bitboard_from_col(col:Col) -> Bitboard {
        0x0101010101010101 << col
    }
}

#[derive(Clone, Debug)]
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

// Template symbols ┏┓╄┗┺┩┛╏╍╌╎└┘┲
///
/// ## Border Template
///
/// const for `Board::to_string`
///
/// Assumes 8 * 8 matrix due to macro/const_fn limitations
///
/// If `Board::ROWS` or `Board::COLS` is changed
/// - `rusty_chess::board::BOARD_TEMPLATE` will need to be fixed,
/// - the `rusty_chess:board::format_board!` macro expansion will need to be fixed as well.
///
const BOARD_TEMPLATE: &'static str =
"╌╌╌┲╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍┓\n
 {} ╏ {}  {}  {}  {}  {}  {}  {}  {} ╏\n
 {} ╏ {}  {}  {}  {}  {}  {}  {}  {} ╏\n
 {} ╏ {}  {}  {}  {}  {}  {}  {}  {} ╏\n
 {} ╏ {}  {}  {}  {}  {}  {}  {}  {} ╏\n
 {} ╏ {}  {}  {}  {}  {}  {}  {}  {} ╏\n
 {} ╏ {}  {}  {}  {}  {}  {}  {}  {} ╏\n
 {} ╏ {}  {}  {}  {}  {}  {}  {}  {} ╏\n
 {} ╏ {}  {}  {}  {}  {}  {}  {}  {} ╏\n
╌╌╌╄╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍┩\n
   ╎ {}  {}  {}  {}  {}  {}  {}  {} ╎\n
   └╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┘";

impl Board {

    /// Number of Rows on board
    const ROWS: u8 = 8;
    /// Number of Columns on board
    const COLS: u8 = 8;

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
            castling_rights : CastlingRights::ALL_CAN_CASTLE,
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
            castling_rights : CastlingRights::NONE_CAN_CASTLE,
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
            castling_rights : self.castling_rights,
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
        for piece in Piece::iter() {
            if self.data[piece as usize] & (1 << square) != 0 {
                return Some(piece);
            }
        }
        None
    }
    pub fn get_piece_color_at(&self, square: Square) -> Option<Color> {
        if self.piece_locations & (1 << square) == 0 {
            return None
        }
        for piece in Piece::iter() {
            if self.data[piece as usize] & (1 << square) != 0 {
                return Some(piece.get_color());
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
    pub fn is_square_occupied(&self, square: Square) -> bool {
        self.piece_locations & (1 << square) != 0
    }
    /// Checks if piece at a given square
    ///
    /// # Arguments
    ///
    /// * `square` - The square to query.
    /// * `piece` - The piece to check.
    ///
    /// # Returns
    ///
    /// `bool` - if the piece at the square
    pub fn is_piece_at(&self, square: Square, piece: Piece) -> bool {
        self.data[piece as usize] & (1 << square) != 0
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
        self.king_square_by_color(self.active_player)
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
    pub fn king_square_by_color(&self, color: Color) -> Option<Square> {
        match match color {
            Color::White => self.data[Piece::WhiteKing as usize].trailing_zeros() as Square,
            Color::Black => self.data[Piece::BlackKing as usize].trailing_zeros() as Square
        } {
            Square::MAX_SQUARES => None,
            square => Some(square),
        }
    }

    /// Is the king of a given color in check?
    ///
    /// # Arguments
    ///
    /// * `color` - The color of the king to check if is check.
    ///
    /// # Returns
    ///
    /// `true` - If there is a piece that threatens the king
    /// `false` - The king is not threatened by any pieces
    pub fn is_in_check(&self, color: Color) -> bool {
        if let Some(king_square) = self.king_square_by_color(color) {
            self.is_threatened(king_square)
        } else {
            false
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
            piece.to_char()
        } else {
            ' '
        }
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
            piece.to_symbol()
        } else {
            '-'
        }
    }
    /// Renders the board as a formatted string.
    ///
    /// # Returns
    ///
    /// A string representing the board layout.
    pub fn to_string(&self) -> String {
        // Template symbols ┏┓╄┗┺┩┛╏╍╌╎└┘
        let mut rendered_board = "┌╌╌╌┲╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍┓\n".to_string();
        for row in Square::iter_ranks() {
            let square: Square = row * 8;// Compute the starting index for this rank
            rendered_board.push_str(&format!(
                "╎ {} ╏ {} {} {} {} {} {} {} {} ╏\n",
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
        rendered_board.push_str("└╌╌╌╄╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍┩\n");
        rendered_board.push_str("    ╎ a b c d e f g h ╎\n");
        rendered_board.push_str("    └╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┘");
        rendered_board
    }

    /// Checks if the given square is threatened by an opponent's piece.
    ///
    /// This method evaluates potential threats from all opponent piece types,
    /// including attacks along ranks, files, diagonals, and knight moves.
    ///
    /// # Parameters
    /// - `square`: The board position to check.
    ///
    /// # Returns
    /// - `true` if the square is under attack.
    /// - `false` if the square is safe.
    pub fn is_threatened(&self, square: Square) -> bool {
        let opponent = match self.get_piece_at(square) {
            Some(piece) => piece.get_opponent_color(),
            None => return false,
        };

        fn captures_straight(board: &Board, distance: u8, square: Square, opponent: Color) -> Option<bool> {
            if board.piece_locations & (1 << square) == 0 {
                return None;
            }
            if let Some(piece)= board.get_colored_piece_at(square, opponent) {match piece{
                Piece::WhiteKing | Piece::BlackKing => {
                    Some(distance==1)
                }
                Piece::WhiteQueen | Piece::BlackQueen | Piece::WhiteRook | Piece::BlackRook => {
                    Some(true)
                }
                _ => Some(false)
            }} else {
                Some(false)
            }
        }
        fn captures_diagonally(board: &Board, distance: u8, square: Square, opponent: Color, ascending_row: bool) -> Option<bool> {
            if board.piece_locations & (1 << square) == 0 {
                return None;
            }
            if let Some(piece)= board.get_colored_piece_at(square, opponent) {match piece{
                Piece::WhiteQueen | Piece::BlackQueen | Piece::WhiteBishop | Piece::BlackBishop => {
                    Some(true)
                }
                Piece::WhitePawn => {
                    if distance == 1 && ascending_row == false {
                        Some(true)
                    } else {
                        Some(false)
                    }
                }
                Piece::BlackPawn => {
                    if distance == 1 && ascending_row == true {
                        Some(true)
                    } else {
                        Some(false)
                    }
                }
                Piece::WhiteKing | Piece::BlackKing => {
                    if distance == 1 {
                        Some(true)
                    } else {
                        Some(false)
                    }
                }
                _ => Some(false)
            }} else {
                Some(false)
            }
        }

        //Check ranks and file
        for i in 0..2 {
            let ascending = i== 1;
            let mut distance = 1;
            for s in square.get_row_squares(ascending) {
                let threatening_square = captures_straight(self, distance, s, opponent);
                match threatening_square {
                    Some(true) => { return true; }
                    Some(false) => { break; }
                    None => { distance += 1; }
                }
            }
            distance = 1;
            for s in square.get_col_squares(ascending) {
                let threatening_square = captures_straight(self, distance, s, opponent);
                match threatening_square {
                    Some(true) => { return true; }
                    Some(false) => { break; }
                    None => { distance += 1; }
                }
            }
        }
        //Check diagonals
        for i in 0..4 {
            let ascending_row = i/2 == 0;
            let ascending_col = i%2 == 0;

            let mut distance = 1;
            for diagonal_square in square.iter_diagonal(ascending_row, ascending_col) {
                let threatening_square = captures_diagonally(self, distance, diagonal_square, opponent, ascending_row);
                match threatening_square {
                    Some(true) => { return true; }
                    Some(false) => { break; }
                    None => { distance += 1; }
                }
            }
        }
        //Check knights
        let enemy_knight = opponent.get_knight();
        for knight_square in square.iter_knight_offsets() {
            if self.is_piece_at(knight_square, enemy_knight) {
                return true;
            }
        }

        false
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
        for diagonal_square in square.iter_diagonal(ascending_row, ascending_col) {
            vision_board |= 1 << diagonal_square;
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
        for knight_square in square.iter_knight_offsets(){
            vision_board |= 1 << knight_square;
        }
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
        for diagonal_square in square.iter_diagonal(ascending_row, ascending_col) {
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
            let pos: Square = bitboard.trailing_zeros() as u8;
            if let Some(piece) = self.get_piece_at(pos) {pieces.push((piece,pos));}
            bitboard &= !(1<<pos);
        }
        pieces
    }
    pub fn get_piece_squares_from_bitboard(&self, piece: Piece, mut bitboard: Bitboard) -> Vec<Square> {
        let mut squares: Vec<Square> = Vec::new();
        while bitboard != 0 {
            let pos: Square = bitboard.trailing_zeros() as u8;
            if self.is_piece_at(pos, piece) {squares.push(pos);}
            bitboard &= !(1 << pos);
        }
        squares
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

impl std::fmt::Display for Board {
    /// Formats the board for display.
    ///
    /// This implementation renders the board using `to_string()`.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.to_string().fmt(f)
    }
}

