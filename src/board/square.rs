use super::Board;

use std::ops::{Add, AddAssign, Sub, SubAssign,
    Mul, MulAssign, Div, DivAssign, Rem, RemAssign};
/// A square on the chessboard represented as a value between 0 and 63 (inclusive).
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct Square(u8);
/// A row on the chessboard represented as a value between 0 and 7 (inclusive).
/// ***Not***, to be confused with a Rank.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct Row(u8);
/// A column on the chessboard represented as a value between 0 and 7 (inclusive).
/// ***Not***, to be confused with a File.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct Col(u8);
/// A square offset for constant definitions
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct OffsetSquare(u8);

/// Provides extended functionality for chess board squares.
pub trait SquareExt {
    /// Size of Square bounds
    const MAX_SQUARES: Square;
    /// Number of Rows
    const ROWS: u8;
    /// Number of Columns
    const COLS: u8;
    /// Squares
    const A1: Square; const A2: Square; const A3: Square; const A4: Square; const A5: Square; const A6: Square; const A7: Square; const A8: Square;
    const B1: Square; const B2: Square; const B3: Square; const B4: Square; const B5: Square; const B6: Square; const B7: Square; const B8: Square;
    const C1: Square; const C2: Square; const C3: Square; const C4: Square; const C5: Square; const C6: Square; const C7: Square; const C8: Square;
    const D1: Square; const D2: Square; const D3: Square; const D4: Square; const D5: Square; const D6: Square; const D7: Square; const D8: Square;
    const E1: Square; const E2: Square; const E3: Square; const E4: Square; const E5: Square; const E6: Square; const E7: Square; const E8: Square;
    const F1: Square; const F2: Square; const F3: Square; const F4: Square; const F5: Square; const F6: Square; const F7: Square; const F8: Square;
    const G1: Square; const G2: Square; const G3: Square; const G4: Square; const G5: Square; const G6: Square; const G7: Square; const G8: Square;
    const H1: Square; const H2: Square; const H3: Square; const H4: Square; const H5: Square; const H6: Square; const H7: Square; const H8: Square;
    /// Array of square labels, from "a8","b8","c8" ... to ..., "f1", "g1", "h1".
    const SQUARES: [&'static str; 64];
    /// Returns the row (0-based) for the square.
    const KNIGHT_OFFSETS: &'static[(Row, Col); 8];
    const KING_OFFSETS: &'static[(Row, Col); 8];
    fn get_row(&self) -> Row;
    /// Returns an iterator over rows relative to this square.
    ///
    /// If `ascending` is true, the iterator starts at the next row up to the top (row 7).
    /// Otherwise, it iterates from row 0 up to the current row (exclusive).
    fn get_rows(&self, ascending : bool) -> RowIterator;
    /// Returns an iterator over squares in the same column (file) in the given direction.
    fn get_row_squares(&self, ascending : bool) -> SquareIterator;
    /// Returns the column (0-based) for the square.
    fn get_col(&self) -> Col;
    /// Returns an iterator over columns relative to this square.
    fn get_cols(&self, ascending : bool) -> ColIterator;
    /// Returns an iterator over squares in the same row (rank) in the given direction.
    fn get_col_squares(&self, ascending : bool) -> SquareIterator;
    /// Returns a tuple of (row, column) for the square.
    fn get_pos_pair(&self) -> (Row, Col);
    /// Returns the index of the square as a `usize`.
    fn get_index(&self) -> usize;
    /// Returns the file (column letter, e.g., 'a' through 'h') for the square.
    fn get_file(&self) -> char;
    /// Returns the rank (1-based, e.g., 1 through 8) for the square.
    fn get_rank(&self) -> u8;
    /// Creates a new square from the given row and column.
    fn new(row : Row, col : Col) -> Square;
    /// Creates a new square from the given row and column if they are valid, otherwise returns `None`.
    fn valid_new(row : Row, col : Col) -> Option<Square>;
    /// Returns the absolute difference is rows between `Square`s
    fn row_diff(&self, other_square: Square) -> u8;
    /// Returns the absolute difference in columns between `Square`s
    fn col_diff(&self, other_square: Square) -> u8;
    /// Returns an iterator over the file letters.
    fn iter_files() -> impl Iterator<Item = char>;
    /// Returns an iterator over the rank numbers.
    fn iter_ranks() -> impl Iterator<Item = u8>;
    /// Returns an iterator over all squares (0 to 63).
    fn iter_squares() -> impl Iterator<Item = Square>;
    /// Returns an iterator over the squares a knight can reach from the Square
    fn iter_knight_offsets(&self) -> impl Iterator<Item = Square>;
    fn iter_king_offsets(&self) -> impl Iterator<Item=Square>;
    /// Returns the string slice representation of the square (e.g., "e4").
    fn to_square_str(&self) -> &str;
    /// Returns the string representation of the square.
    fn to_square_string(&self) -> String;
    fn iter_diagonal(&self, ascending_row: bool, ascending_col: bool) -> DiagonalSquareIterator;
}
impl Square {
    const MAX_SQUARES: u8 = Square::ROWS * Square::COLS;
    const ROWS: u8 = Board::ROWS;
    const COLS: u8 = Board::COLS;
    const A8: Square = Square(0_); const B8: Square = Square(1_); const C8: Square = Square(2_); const D8: Square = Square(3_); const E8: Square = Square(4_); const F8: Square = Square(5_); const G8: Square = Square(6_); const H8: Square = Square(7_);
    const A7: Square = Square(8_); const B7: Square = Square(9_); const C7: Square = Square(10); const D7: Square = Square(11); const E7: Square = Square(12); const F7: Square = Square(13); const G7: Square = Square(14); const H7: Square = Square(15);
    const A6: Square = Square(16); const B6: Square = Square(17); const C6: Square = Square(18); const D6: Square = Square(19); const E6: Square = Square(20); const F6: Square = Square(21); const G6: Square = Square(22); const H6: Square = Square(23);
    const A5: Square = Square(24); const B5: Square = Square(25); const C5: Square = Square(26); const D5: Square = Square(27); const E5: Square = Square(28); const F5: Square = Square(29); const G5: Square = Square(30); const H5: Square = Square(31);
    const A4: Square = Square(32); const B4: Square = Square(33); const C4: Square = Square(34); const D4: Square = Square(35); const E4: Square = Square(36); const F4: Square = Square(37); const G4: Square = Square(38); const H4: Square = Square(39);
    const A3: Square = Square(40); const B3: Square = Square(41); const C3: Square = Square(42); const D3: Square = Square(43); const E3: Square = Square(44); const F3: Square = Square(45); const G3: Square = Square(46); const H3: Square = Square(47);
    const A2: Square = Square(48); const B2: Square = Square(49); const C2: Square = Square(50); const D2: Square = Square(51); const E2: Square = Square(52); const F2: Square = Square(53); const G2: Square = Square(54); const H2: Square = Square(55);
    const A1: Square = Square(56); const B1: Square = Square(57); const C1: Square = Square(58); const D1: Square = Square(59); const E1: Square = Square(60); const F1: Square = Square(61); const G1: Square = Square(62); const H1: Square = Square(63);
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
    const KNIGHT_OFFSETS: &'static[(Row, Col); 8] = &[
        (254u8, 255u8), (254u8, 1u8),
        (255u8, 254u8), (255u8, 2u8),
        (1u8, 254u8), (1u8, 2u8),
        (2u8, 255u8), (2u8, 1u8)
    ];
    const KING_OFFSETS: &'static [(Row, Col); 8] = &[
        (255, 255), (255, 0), (255, 1),
        (0, 255), (0, 1),
        (1, 255), (1, 0), (1, 1),
    ];
    pub(crate) fn get_row(self) -> Row {
        Row(self.0 / Square::ROWS)
    }
    fn get_rows(&self, ascending : bool) -> RowIterator {
        RowIterator{
            row: self.get_row(),
            ascending,
        }
    }
    fn get_row_squares(&self, ascending : bool) -> SquareIterator {
        if ascending {
            SquareIterator{
                square: *self,
                next_square_offset: Square::COLS as i8,
                squares_remaining: Square::COLS -1 - self.get_row(),
            }
        } else{
            SquareIterator{
                square: *self,
                next_square_offset: 0i8 - Self::COLS as i8,
                squares_remaining: self.get_row(),
            }
        }
    }
    fn get_col(&self) -> Col {
        Col(self.0 % Square::COLS)
    }
    fn get_cols(&self, ascending : bool) -> ColIterator {
        ColIterator{
            col: self.get_col(),
            ascending,
        }
    }
    fn get_col_squares(&self, ascending : bool) -> SquareIterator {
        if ascending {
            SquareIterator{
                square: *self,
                next_square_offset: 1,
                squares_remaining: Square::COLS -1 - self.get_col(),
            }
        } else {
            SquareIterator{
                square: *self,
                next_square_offset: -1,
                squares_remaining: self.get_col(),
            }
        }
    }
    fn get_pos_pair(&self) -> (Row, Col) {
        (self.get_row(),self.get_col())
    }
    fn get_index(&self) -> usize {
        *self as usize
    }
    fn get_file(&self) -> char {
        match self % Square::COLS {
            0 => 'a', 1 => 'b', 2 => 'c', 3 => 'd', 4 => 'e', 5 => 'f', 6 => 'g', 7 => 'h',
            _ => panic!("Invalid file index"),
        }
    }
    fn get_rank(&self) -> u8 {
        Square::ROWS - self / Square::ROWS
    }
    fn new(row : Row, col : Col) -> Square {
        row * Square::COLS + col
    }
    fn valid_new(row : Row, col : Col) -> Option<Square> {
        if row>= Square::ROWS || col >= Square::COLS {None} else {Some(Square::new(row, col))}
    }

    fn row_diff(&self, other_square: Square) -> u8 {
        let self_minus_other = self.get_row() - other_square.get_row();
        if self_minus_other < Square::ROWS {
            self_minus_other
        } else {
            other_square.get_row() - self.get_row()
        }
    }
    fn col_diff(&self, other_square: Square) -> u8 {
        let diff = self.get_col() - other_square.get_col();
        if diff < 64 {
            diff
        } else {
            0 - diff
        }
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
    fn iter_knight_offsets(&self) -> impl Iterator<Item=Square> {
        KnightIterator{
            row: self.get_row(),
            col: self.get_col(),
            index: 0,
        }
    }
    fn iter_king_offsets(&self) -> impl Iterator<Item=Square> {
        KingIterator{
            row: self.get_row(),
            col: self.get_col(),
            index: 0,
        }
    }

    fn to_square_str(&self) -> &str {
        Self::SQUARES[*self as usize]
    }
    fn to_square_string(&self) -> String {
        format!("{}{}", self.get_file(),self.get_rank())
    }
    fn iter_diagonal(&self, ascending_row: bool, ascending_col: bool) -> DiagonalSquareIterator {
        DiagonalSquareIterator{
            row : self.get_row(), 
            col : self.get_col(),
            ascending_row,
            ascending_col,
        }
    }
}
pub trait RowExt {
    const MAX_ROW: u8;
    fn new_row(row_as_u8: u8) -> Option<Row>;
    ///
    ///
    /// # Arguments
    ///
    /// * `ascending`: whether you want the next value to be greater (if true) or lesser
    ///   (if false) than the starting row.
    ///
    /// returns: Option<Row>
    ///
    /// # Examples
    /// ```
    /// let r:Row = 2; assert!(r.get_next_row(true), 3); assert!(r.get_next_row(false), 1);
    /// let r:Row = 5; assert!(r.get_next_row(true), 6); assert!(r.get_next_row(false), 4);
    /// ```
    fn get_next_row(&self, ascending: bool) -> Option<Row>;
    fn from_rank(rank: u8) -> Self;
    fn to_rank(self) -> u8;
}
pub trait ColExt {
    const MAX_COL: u8;
    fn new_col(col_as_u8: u8) -> Option<Col>;
    ///
    ///
    /// # Arguments
    ///
    /// * `ascending`: whether you want the next value to be greater (if true) or lesser
    ///   (if false) than the starting row.
    ///
    /// returns: Option<Row>
    ///
    /// # Examples
    /// ```
    /// let c:Col = 2; assert!(c.get_next_col(true), 3); assert!(c.get_next_col(false), 1);
    /// let c:Col = 5; assert!(c.get_next_col(true), 6); assert!(c.get_next_col(false), 4);
    /// ```
    fn get_next_col(&self, ascending: bool) -> Option<Col>;
    fn from_file(file: char) -> Self;
    fn to_file(self) -> char;
}
impl Row {
    const MAX_ROW: u8 = Square::ROWS - 1;
    fn new_row(row: u8) -> Option<Row> {
        if row <= Row::MAX_ROW {
            return Some(Row(row))
        }
        None
    }
    fn get_next_row(&self, ascending: bool) -> Option<Row> {
        let next = match ascending {
            true => self.wrapping_add(1),
            false => self.wrapping_sub(1),
        };
        Row::new_row(next)
    }
    fn from_rank(rank: u8) -> Row {
        match rank % Square::ROWS {
            0 => 0 as Row,
            rank => Square::ROWS - rank,
        }
    }
    fn to_rank(self) -> u8 {
        Square::ROWS - self
    }
}
impl ColExt for Col {
    const MAX_COL: u8 = Square::COLS - 1;
    fn new_col(col: u8) -> Option<Col> {
        if col <= Col::MAX_COL {
            return Some(col as Col)
        }
        None
    }
    fn get_next_col(&self, ascending: bool) -> Option<Col> {
        let next = match ascending {
            true => self.wrapping_add(1),
            false => self.wrapping_sub(1),
        };
        Col::new_row(next)
    }
    fn from_file(file: char) -> Col {
        if file >= 'a' {
            ((file as u8 - b'a') % Square::COLS) as u8
        } else {
            ((file as u8 - b'A') % Square::COLS) as u8
        }
    }
    fn to_file(self) -> char {
        (b'a' as u8 + self) as char
    }
}

impl Row {
    const NEG_ONE: u8 = u8::MAX;

    fn new(row: i8) -> Row {
        let r = row % Square::ROWS;
        Row(
            match r {
                0..Square::ROWS => r as u8,
                neg => Self::NEG_ONE -
            }
        )
    }
}
impl Col {

}

impl OffsetSquare {
    const NEG_ONE: u8 = u8::MAX;
    
    const fn from_row(row: Row) -> OffsetSquare {
        OffsetSquare(
            row.0
            .wrapping_mul(Square::COLS)
        )
    }
    const fn from_col(col: Col) -> OffsetSquare {
        OffsetSquare(col.0)
    }
    const fn new(row: Row, col: Col) -> OffsetSquare {
        OffsetSquare(
            row.0
            .wrapping_mul(Square::COLS)
            .wrapping_add(col.0)
        )
    }
}

pub(crate) mod square_arithmetic {
    use super::{Square,Col,Row};
    use super::{Add, AddAssign, Sub, SubAssign,
    Mul, MulAssign, Div, DivAssign, Rem, RemAssign};

    //impl Add for Square {
    //    type Output = Square;
    //    fn add(self, rhs: Square) -> Self::Output {
    //        let mut value = self.0 + rhs.0;
    //        value %= Self::MAX_SQUARES;
    //        Self(value)
    //    }
    //}
    impl Add<Row> for Square {
        type Output = Square;
        fn add(self, rhs: Row) -> Self::Output {
            let mut value = self.0 + rhs.0 * Self::COLS;
            value %= Self::MAX_SQUARES;
            Self(value)
        }
    }
    impl Add<Col> for Square {
        type Output = Square;
        fn add(self, rhs: Col) -> Self::Output {
            let mut value = self.0 + rhs.0 * Self::COLS;
            value %= Self::MAX_SQUARES;
            Self(value)
        }
    }
    impl Sub<Row> for Square {
        type Output = Square;
        fn sub(self, rhs: Row) -> Self::Output {
            Self(self.0 - rhs.0 * Square::MAX_SQUARES)
        }
    }
    impl Sub<Col> for Square {
        type Output = Square;
        fn sub(self, rhs: Col) -> Self::Output {
            Self(self.0 - rhs.0)
        }
    }

    macro_rules! impl_op {
        {$t:ty, $rhs:ty, $op:ident, $fn:ident, $equation:expr} => {
            impl $op<$rhs> for $t {
                type Output = $t;
                fn $fn(self, rhs: $rhs) -> Self::Output {
                    $t::new($equation(self, rhs))
                }
            }
        };
    }

    impl_op!{Row, u8, Mul, mul, |x, y| (x.0.wrapping_mul(y))}
}
pub(crate) mod square_offset_arithmetic {
    use super::{Square, OffsetSquare, Row, Col};
    use super::{Add,AddAssign,Sub,SubAssign};

    impl Add<OffsetSquare> for Square {
        type Output = Option<Square>;
        fn add(self, rhs: OffsetSquare) -> Self::Output {
            let sum = self.0 + rhs.0;
            match sum {
                0..Square::MAX_SQUARES => Some(Square(sum)),
                _invalid => None,
            }
        }
    }

    impl Add for OffsetSquare {
        type Output = OffsetSquare;
        fn add(self, rhs: OffsetSquare) -> Self::Output {
            OffsetSquare(
                self.0
                .wrapping_add(rhs.0)
            )
        }
    }
    impl AddAssign for OffsetSquare {
        fn add_assign(&mut self, rhs: OffsetSquare) {
            self.0 = self.0
                .wrapping_add(rhs.0)
        }
    }

    impl Add<Row> for OffsetSquare {
        type Output = OffsetSquare;
        fn add(self, rhs: Row) -> Self::Output {
            OffsetSquare(
                self.0
                .wrapping_add(
                    rhs.0 * Square::COLS
                )
            )
        }
    }
    impl AddAssign<Row> for OffsetSquare {
        fn add_assign(&mut self, rhs: Row) {
            self.0 = self.0
                .wrapping_add(
                    rhs.0 * Square::COLS
                );
        }
    }
    impl Sub<Row> for OffsetSquare {
        type Output = OffsetSquare;
        fn sub(self, rhs: Row) -> Self::Output {
            OffsetSquare(
                self.0
                .wrapping_add(
                    rhs.0 * Square::COLS
                )
            )
        }
    }
    impl SubAssign<Row> for OffsetSquare {
        fn sub_assign(&mut self, rhs: Row) {
            self.0 = self.0
                .wrapping_sub(
                    rhs.0 * Square::COLS
                );
        }
    }

    impl Add<Col> for OffsetSquare {
        type Output = OffsetSquare;
        fn add(self, rhs: Col) -> Self::Output {
            OffsetSquare(
                self.0
                .wrapping_add(rhs.0)
            )
        }
    }
    impl AddAssign<Col> for OffsetSquare {
        fn add_assign(&mut self, rhs: Col) {
            self.0 = self.0
                .wrapping_add(rhs.0);
        }
    }
    impl Sub<Col> for OffsetSquare {
        type Output = OffsetSquare;
        fn sub(self, rhs: Col) -> Self::Output {
            OffsetSquare(
                self.0
                .wrapping_sub(rhs.0)
            )
        }
    }
    impl SubAssign<Col> for OffsetSquare {
        fn sub_assign(&mut self, rhs: Col) {
            self.0 = self.0
                .wrapping_sub(rhs.0);
        }
    }
}

pub struct SquareIterator{
    square: Square,
    next_square_offset: i8,
    squares_remaining : u8,
}
pub struct DiagonalSquareIterator{
    row: Row,
    col: Col,
    ascending_row: bool,
    ascending_col: bool,
}
pub struct RowIterator{
    row: Row,
    ascending: bool
}
pub struct ColIterator{
    col: Col,
    ascending: bool
}
pub struct KnightIterator{
    row: Row,
    col: Col,
    index: usize,
}
pub struct KingIterator{
    row: Row,
    col: Col,
    index: usize,
}
impl Iterator for SquareIterator {
    type Item = Square;
    fn next(&mut self) -> Option<Self::Item> {
       if self.squares_remaining == 0 {
           return None
       }
       self.square = self.square.wrapping_add(self.next_square_offset as u8);
       self.squares_remaining -= 1;
       Some(self.square)
    }
}
impl Iterator for DiagonalSquareIterator {
    type Item = Square;
    fn next(&mut self) -> Option<Self::Item> {
        self.row = match self.row.get_next_row(self.ascending_row) {
            Some(value) => value,
            None => return None,
        };
        self.col = match self.col.get_next_col(self.ascending_col) {
            Some(value) => value,
            None => return None,
        };
        Some(Square::new(self.row, self.col))
    }
}
impl Iterator for RowIterator {
    type Item = Row;
    fn next(&mut self) -> Option<Self::Item> {
        match self.row.get_next_row(self.ascending) {
            Some(row) => {
                self.row = row;
                Some(row)
            },
            None => None,
        }
    }
}
impl Iterator for ColIterator {
    type Item = Col;
    fn next(&mut self) -> Option<Self::Item> {
        match self.col.get_next_col(self.ascending) {
            Some(col) => {
                self.col = col;
                Some(col)
            },
            None => None,
        }
    }
}
impl Iterator for KnightIterator {
    type Item = Square;
    fn next(&mut self) -> Option<Self::Item> {
        while self.index < <u8 as SquareExt>::KNIGHT_OFFSETS.len() {
            let offset = <u8 as SquareExt>::KNIGHT_OFFSETS[self.index];
            self.index += 1;
            match Square::valid_new(self.row + offset.0, self.col + offset.1) {
                Some(square) => return Some(square),
                None => continue,
            }
        }
        None
    }
}
impl Iterator for KingIterator {
    type Item = Square;
    fn next(&mut self) -> Option<Self::Item> {
        while self.index < <u8 as SquareExt>::KING_OFFSETS.len() {
            let offset = <u8 as SquareExt>::KING_OFFSETS[self.index];
            self.index += 1;
            match Square::valid_new(self.row + offset.0, self.col + offset.1) {
                Some(square) => return Some(square),
                None => continue,
            }
        }
        None
    }
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.to_square_string().fmt(f)
    }
}
