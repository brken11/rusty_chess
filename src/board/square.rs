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
/// A row offset for square arithmetic
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct OffsetRow(u8);
/// A column offset for square arithmetic
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct OffsetCol(u8);

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
    /// Size of Square bounds
    pub const MAX_SQUARES: u8 = Square::ROWS * Square::COLS;
    /// Number of Rows
    pub const ROWS: u8 = Board::ROWS;
    /// Number of Columns
    pub const COLS: u8 = Board::COLS;
    /// Squares
    pub const A8: Square = Square(0_); pub const B8: Square = Square(1_); pub const C8: Square = Square(2_); pub const D8: Square = Square(3_); pub const E8: Square = Square(4_); pub const F8: Square = Square(5_); pub const G8: Square = Square(6_); pub const H8: Square = Square(7_);
    pub const A7: Square = Square(8_); pub const B7: Square = Square(9_); pub const C7: Square = Square(10); pub const D7: Square = Square(11); pub const E7: Square = Square(12); pub const F7: Square = Square(13); pub const G7: Square = Square(14); pub const H7: Square = Square(15);
    pub const A6: Square = Square(16); pub const B6: Square = Square(17); pub const C6: Square = Square(18); pub const D6: Square = Square(19); pub const E6: Square = Square(20); pub const F6: Square = Square(21); pub const G6: Square = Square(22); pub const H6: Square = Square(23);
    pub const A5: Square = Square(24); pub const B5: Square = Square(25); pub const C5: Square = Square(26); pub const D5: Square = Square(27); pub const E5: Square = Square(28); pub const F5: Square = Square(29); pub const G5: Square = Square(30); pub const H5: Square = Square(31);
    pub const A4: Square = Square(32); pub const B4: Square = Square(33); pub const C4: Square = Square(34); pub const D4: Square = Square(35); pub const E4: Square = Square(36); pub const F4: Square = Square(37); pub const G4: Square = Square(38); pub const H4: Square = Square(39);
    pub const A3: Square = Square(40); pub const B3: Square = Square(41); pub const C3: Square = Square(42); pub const D3: Square = Square(43); pub const E3: Square = Square(44); pub const F3: Square = Square(45); pub const G3: Square = Square(46); pub const H3: Square = Square(47);
    pub const A2: Square = Square(48); pub const B2: Square = Square(49); pub const C2: Square = Square(50); pub const D2: Square = Square(51); pub const E2: Square = Square(52); pub const F2: Square = Square(53); pub const G2: Square = Square(54); pub const H2: Square = Square(55);
    pub const A1: Square = Square(56); pub const B1: Square = Square(57); pub const C1: Square = Square(58); pub const D1: Square = Square(59); pub const E1: Square = Square(60); pub const F1: Square = Square(61); pub const G1: Square = Square(62); pub const H1: Square = Square(63);
    /// Array of square labels, from "a8","b8","c8" ... to ..., "f1", "g1", "h1".
    pub const SQUARES: [&'static str; 64] = [
        "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8",
        "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7",
        "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
        "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5",
        "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
        "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3",
        "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
        "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1",
    ];
    pub const KNIGHT_OFFSETS: &'static[(OffsetRow, OffsetCol); 8] = &[
        (OffsetRow::new(-2), OffsetCol::new(-1)), (OffsetRow::new(-2), OffsetCol::new(1)),
        (OffsetRow::new(-1), OffsetCol::new(-2)), (OffsetRow::new(-1), OffsetCol::new(2)),
        (OffsetRow::new(1), OffsetCol::new(-2)), (OffsetRow::new(1), OffsetCol::new(2)),
        (OffsetRow::new(2), OffsetCol::new(-1)), (OffsetRow::new(2), OffsetCol::new(1)),
    ];
    pub const KING_OFFSETS: &'static [(OffsetRow, OffsetCol); 8] = &[
        (OffsetRow::new(-1), OffsetCol::new(-1)), (OffsetRow::new(-1), OffsetCol::new(0)), (OffsetRow::new(-1), OffsetCol::new(1)),
        (OffsetRow::new(0), OffsetCol::new(-1)), (OffsetRow::new(0), OffsetCol::new(1)),
        (OffsetRow::new(1), OffsetCol::new(-1)), (OffsetRow::new(1), OffsetCol::new(0)), (OffsetRow::new(1), OffsetCol::new(1)),
    ];
    /// Returns the row (0-based) for the square.
    pub(crate) const fn get_row(self) -> Row {
        Row(self.0 / Square::ROWS)
    }
    /// Returns an iterator over rows relative to this square.
    ///
    /// If `ascending` is true, the iterator starts at the next row up to the top (row 7).
    /// Otherwise, it iterates from row 0 up to the current row (exclusive).
    pub const fn get_rows(&self, ascending : bool) -> RowIterator {
        RowIterator{
            row: self.get_row(),
            ascending,
        }
    }
    /// Returns an iterator over squares in the same column (file) in the given direction.
    pub fn get_row_squares(self, ascending : bool) -> SquareIterator {
        if ascending {
            SquareIterator{
                square: self,
                next_square_offset: OffsetSquare::from_row(OffsetRow::POS_ONE),
                squares_remaining: Row::MAX_ROW - self.get_row().get_next_rows(ascending),
            }
        } else{
            SquareIterator{
                square: self,
                next_square_offset: OffsetSquare::from_row(OffsetRow::NEG_ONE),
                squares_remaining: self.get_row().get_next_rows(ascending),
            }
        }
    }
    /// Returns the column (0-based) for the square.
    pub const fn get_col(self) -> Col {
        Col(self.0 % Square::COLS)
    }
    /// Returns an iterator over columns relative to this square.
    pub const fn get_cols(self, ascending : bool) -> ColIterator {
        ColIterator{
            col: self.get_col(),
            ascending,
        }
    }
    /// Returns an iterator over squares in the same row (rank) in the given direction.
    pub const fn get_col_squares(self, ascending : bool) -> SquareIterator {
        if ascending {
            SquareIterator{
                square: self,
                next_square_offset: OffsetSquare::from_col(OffsetCol::new(1)),
                squares_remaining: self.get_col().get_next_cols(ascending),
            }
        } else {
            SquareIterator{
                square: self,
                next_square_offset: OffsetSquare::from_col(OffsetCol::new(-1)),
                squares_remaining: self.get_col().get_next_cols(ascending),
            }
        }
    }
    /// Returns a tuple of (row, column) for the square.
    pub const fn get_pos_pair(&self) -> (Row, Col) {
        (self.get_row(),self.get_col())
    }
    /// Returns the index of the square as a `usize`.
    pub const fn get_index(&self) -> usize {
        self.0 as usize
    }
    /// Returns the file (column letter, e.g., 'a' through 'h') for the square.
    pub const fn get_file(self) -> char {
        self.get_col()
            .to_file()
    }
    /// Returns the rank (1-based, e.g., 1 through 8) for the square.
    pub const fn get_rank(&self) -> u8 {
        self.get_row()
            .to_rank()
    }
    /// Creates a new square from the given row and column.
    pub(crate) const fn new(row : Row, col : Col) -> Square {
        Square(
            row.0 * Square::COLS
            + col.0
        )
    }
    /// Creates a new square from the given row and column if they are valid, otherwise returns `None`.
    pub fn valid_new(row : u8, col : u8) -> Option<Square> {
        if (Row::MIN_ROW..=Row::MAX_ROW).contains(&row)
            && (Col::MIN_COL..=Col::MAX_COL).contains(&col) {
            return Some(Square::new(Row(row),Col(col)))
        }
        None
    }
    /// Returns the absolute difference is rows between `Square`s
    pub fn row_diff(self, other_square: Square) -> u8 {
        self.get_row()
            .abs_diff(other_square.get_row())
    }
    /// Returns the absolute difference in columns between `Square`s
    pub fn col_diff(self, other_square: Square) -> u8 {
        self.get_col()
            .abs_diff(other_square.get_col())
    }

    fn get_diagonal_offset(self, ascending_row: bool, ascending_col: bool) -> OffsetSquare {
        OffsetSquare::new(
            OffsetRow::from_ascending(ascending_row),
            OffsetCol::from_ascending(ascending_col),
        )
    }

    /// Returns an iterator over the file letters.
    pub fn iter_files() -> impl Iterator<Item = char> {Col::iter_files()}
    /// Returns an iterator over the rank numbers.
    pub fn iter_ranks() -> impl Iterator<Item = u8> {Row::iter_ranks()}
    /// Returns an iterator over all squares (0 to 63).
    pub fn iter_squares() -> impl Iterator<Item = Square> {
    (0..Square::MAX_SQUARES)
        .into_iter()
        .map(|i| Square(i))
    }
    /// Returns an iterator over the squares a knight can reach from the Square
    pub const fn iter_knight_offsets(&self) -> impl Iterator<Item=Square> {
        KnightIterator{
            row: self.get_row(),
            col: self.get_col(),
            index: 0,
        }
    }
    /// Returns an iterator over the squares a King can reach from the Square
    pub const fn iter_king_offsets(&self) -> impl Iterator<Item=Square> {
        KingIterator{
            row: self.get_row(),
            col: self.get_col(),
            index: 0,
        }
    }

    pub const fn to_square_str(&self) -> &str {
        Self::SQUARES[self.0 as usize]
    }
    pub fn to_square_string(&self) -> String {
        format!("{}{}", self.get_file(),self.get_rank())
    }
    pub fn iter_diagonal(self, ascending_row: bool, ascending_col: bool) -> DiagonalSquareIterator {
        let squares_remaining = self.get_row()
                .get_next_rows(ascending_row)
                .min(
                    self.get_col()
                    .get_next_cols(ascending_col)
                );
        let diagonal_offset = OffsetSquare::new(
            OffsetRow::from_ascending(ascending_row),
            OffsetCol::from_ascending(ascending_col),
        );
        DiagonalSquareIterator{
            square: self,
            squares_remaining,
            diagonal_offset, 
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
    pub const MAX_ROW: u8 = Square::ROWS - 1;
    pub const MIN_ROW: u8 = 0;
    pub const fn new_row(row: u8) -> Option<Row> {
        if row <= Row::MAX_ROW {
            return Some(Row(row))
        }
        None
    }
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
    pub const fn get_next_row(self, ascending: bool) -> Option<Row> {
        let next = match ascending {
            true => self.0.wrapping_add(1),
            false => self.0.wrapping_sub(1),
        };
        Row::new_row(next)
    }
    pub const fn get_next_rows(self, ascending: bool) -> u8 {
        match ascending {
            true => self.0 - Self::MIN_ROW,
            false => Self::MAX_ROW - self.0,
        }
    }
    pub fn iter_rows() -> impl Iterator<Item=Row> {
        (Self::MIN_ROW..=Self::MAX_ROW)
            .into_iter()
            .map(|i| Row(i))
    }
    pub fn iter_ranks() -> impl Iterator<Item=u8> {
        (Square::ROWS..0)
            .into_iter()
    }

    pub const fn abs_diff(self, other_row: Row) -> u8 {
        self.0
            .abs_diff(other_row.0)
    }

    pub const fn from_rank(rank: u8) -> Row {
        match rank % Square::ROWS {
            0 => Row(0),
            rank => Row(Square::ROWS - rank),
        }
    }
    pub const fn to_rank(self) -> u8 {
        Square::ROWS - self.0
    }

    pub const fn as_u8(self) -> u8 {
        self.0
    }
}
impl Col {
    pub const MAX_COL: u8 = Square::COLS - 1;
    pub const MIN_COL: u8 = 0;
    pub const fn new_col(col: u8) -> Option<Col> {
        if col <= Col::MAX_COL {
            return Some(Col(col))
        }
        None
    }
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
    pub const fn get_next_col(self, ascending: bool) -> Option<Col> {
        let next = match ascending {
            true => self.0.wrapping_add(1),
            false => self.0.wrapping_sub(1),
        };
        Col::new_col(next)
    }
    pub const fn get_next_cols(self, ascending: bool) -> u8 {
        match ascending {
            true => self.0 - Self::MIN_COL,
            false => Self::MAX_COL - self.0,
        }
    }
    pub fn iter_cols() -> impl Iterator<Item=Col> {
        (Self::MIN_COL..=Self::MAX_COL)
            .into_iter()
            .map(|i| Col(i))
    }
    pub fn iter_files() -> impl Iterator<Item=char> {
        (Self::MIN_COL..=Self::MAX_COL)
            .into_iter()
            .map(|i| Col(i).to_file())
    }

    pub const fn abs_diff(self, other_col: Col) -> u8 {
        self.0
            .abs_diff(other_col.0)
    }

    pub const fn from_file(file: char) -> Col {
        if file >= 'a' {
            Col(
                (file as u8 - b'a') % Square::COLS
            )
        } else {
            Col(
                (file as u8 - b'A') % Square::COLS
            )
        }
    }
    pub const fn to_file(self) -> char {
        (b'a' as u8 + self.0) as char
    }

    pub const fn as_u8(self) -> u8 {
        self.0
    }
}

impl OffsetRow {
    pub const POS_ONE: OffsetRow = OffsetRow::new(1);
    pub const NEG_ONE: OffsetRow = OffsetRow::new(-1);

    pub const fn new(row_diff: i8) -> OffsetRow {
        OffsetRow((row_diff % Square::ROWS as i8) as u8)
    }
    pub const fn from_ascending(ascending_row: bool) -> Self {
        match ascending_row {
            true => Self::POS_ONE,
            false => Self::NEG_ONE,
        }
    }
}
impl OffsetCol {
    pub const POS_ONE: OffsetCol = OffsetCol::new(1);
    pub const NEG_ONE: OffsetCol = OffsetCol::new(-1);

    pub const fn new(col_diff: i8) -> OffsetCol {
        OffsetCol((col_diff % Square::COLS as i8) as u8)
    }

    pub const fn from_ascending(ascending_col: bool) -> Self {
        match ascending_col {
            true => Self::POS_ONE,
            false => Self::NEG_ONE,
        }
    }
}
impl OffsetSquare {
    const NEG_ONE: u8 = u8::MAX;
    
    const fn from_row(row: OffsetRow) -> Self {
        Self(
            row.0
            .wrapping_mul(Square::COLS)
        )
    }
    const fn from_col(col: OffsetCol) -> Self {
        Self(
            col.0
        )
    }
    const fn new(row: OffsetRow, col: OffsetCol) -> Self {
        Self(
            row.0
            .wrapping_mul(Square::COLS)
            .wrapping_add(col.0)
        )
    }
}

mod local_arithmetic {
    use super::{Square,Col,Row};
    use super::{Add, AddAssign, Sub, SubAssign,
    Mul, MulAssign, Div, DivAssign, Rem, RemAssign};

    impl Add<Row> for u8 {
        type Output = u8;
        fn add(self, rhs: Row) -> Self::Output {self + rhs.0}
    }
    impl AddAssign<Row> for u8 {
        fn add_assign(&mut self, rhs: Row) {*self += rhs.0}
    }
    impl Add<Col> for u8 {
        type Output = u8;
        fn add(self, rhs: Col) -> Self::Output {self + rhs.0}
    }
    impl AddAssign<Col> for u8 {
        fn add_assign(&mut self, rhs: Col) {*self += rhs.0}
    }

    impl Sub<Row> for u8 {
        type Output = u8;
        fn sub(self, rhs: Row) -> Self::Output {self - rhs.0}
    }
    impl SubAssign<Row> for u8 {
        fn sub_assign(&mut self, rhs: Row) {*self -= rhs.0}
    }
    impl Sub<Col> for u8 {
        type Output = u8;
        fn sub(self, rhs: Col) -> Self::Output {self - rhs.0}
    }
    impl SubAssign<Col> for u8 {
        fn sub_assign(&mut self, rhs: Col) {*self -= rhs.0}
    }

    impl Mul<Row> for u8 {
        type Output = u8;
        fn mul(self, rhs: Row) -> Self::Output {self * rhs.0}
    }
    impl MulAssign<Row> for u8 {
        fn mul_assign(&mut self, rhs: Row) {*self *= rhs.0}
    }
    impl Mul<Col> for u8 {
        type Output = u8;
        fn mul(self, rhs: Col) -> Self::Output {self * rhs.0}
    }
    impl MulAssign<Col> for u8 {
        fn mul_assign(&mut self, rhs: Col) {*self *= rhs.0}
    }
}
pub(crate) mod square_arithmetic {
    use super::{Square, OffsetSquare, OffsetRow, OffsetCol, Row, Col};
    use super::{Add, AddAssign, Sub};

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

    impl Add<OffsetRow> for Row {
        type Output = Option<Row>;
        fn add(mut self, rhs: OffsetRow) -> Self::Output {
            let new_row = self.0
                    .wrapping_add(rhs.0);
            if (Row::MIN_ROW..=Row::MAX_ROW).contains(&new_row) {
                self.0 = self.0
                    .wrapping_add(rhs.0 * Square::COLS);
                return Some(self)
            }
            None
        }
    }
    impl Add<OffsetRow> for Square {
        type Output = Option<Square>;
        fn add(mut self, rhs: OffsetRow) -> Self::Output {
            let new_row = self.get_row().0
                    .wrapping_add(rhs.0);
            if (0..=Row::MAX_ROW).contains(&new_row) {
                self.0 = self.0
                    .wrapping_add(rhs.0 * Square::COLS);
                return Some(self)
            }
            None
        }
    }

    impl Add<OffsetCol> for Square {
        type Output = Option<Square>;
        fn add(mut self, rhs: OffsetCol) -> Self::Output {
            let new_col = self.get_col().0
                    .wrapping_add(rhs.0);
            if (0..=Col::MAX_COL).contains(&new_col) {
                self.0 = self.0
                    .wrapping_add(rhs.0 * Square::COLS);
                return Some(self)
            }
            None
        }
    }
    impl Add<OffsetCol> for Col {
        type Output = Option<Col>;
        fn add(mut self, rhs: OffsetCol) -> Self::Output {
            let new_col = self.0
                    .wrapping_add(rhs.0);
            if (Col::MIN_COL..=Col::MAX_COL).contains(&new_col) {
                self.0 = self.0
                    .wrapping_add(rhs.0);
                return Some(self)
            }
            None
        }
    }

    impl Sub for Square {
        type Output = OffsetSquare;
        fn sub(self, rhs: Square) -> Self::Output {
            OffsetSquare(
                self.0
                    .wrapping_sub(rhs.0)
            )
        }
    }
    impl Sub for Row {
        type Output = OffsetRow;
        fn sub(self, rhs: Row) -> Self::Output {
            OffsetRow(
                self.0
                    .wrapping_sub(rhs.0)
            )
        }
    }
    impl Sub for Col {
        type Output = OffsetCol;
        fn sub(self, rhs: Col) -> Self::Output {
            OffsetCol(
                self.0
                    .wrapping_mul(rhs.0)
            )
        }
    }
}

pub struct SquareIterator{
    square: Square,
    next_square_offset: OffsetSquare,
    squares_remaining : u8,
}
pub struct DiagonalSquareIterator{
    square: Square,
    diagonal_offset: OffsetSquare,
    squares_remaining: u8,
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
       
       let next_square = self.square + self.next_square_offset;
       if let Some(square) = next_square {
           self.square = square;
           self.squares_remaining -= 1;
       }
       next_square
    }
}
impl Iterator for DiagonalSquareIterator {
    type Item = Square;
    fn next(&mut self) -> Option<Self::Item> {
        if self.squares_remaining == 0 {
            return None
        }
        
        let next_square = self.square + self.diagonal_offset;
        if let Some(square) = next_square {
            self.square = square;
            self.squares_remaining -= 1;
        }
        next_square
    }
}
impl Iterator for RowIterator {
    type Item = Row;
    fn next(&mut self) -> Option<Self::Item> {
        self.row
            .get_next_row(self.ascending)
            .map(|row| {
                self.row = row;
                row
            })
    }
}
impl Iterator for ColIterator {
    type Item = Col;
    fn next(&mut self) -> Option<Self::Item> {
        self.col
            .get_next_col(self.ascending)
            .map(|col| {
                self.col = col;
                col
            })
    }
}
impl Iterator for KnightIterator {
    type Item = Square;
    fn next(&mut self) -> Option<Self::Item> {
        while self.index < Square::KNIGHT_OFFSETS.len() {
            let (row_offset, col_offset) = Square::KNIGHT_OFFSETS[self.index];
            self.index += 1;

            if let (Some(row), Some(col)) =
                (self.row + row_offset, self.col + col_offset) {
                    return Some(Square::new(row, col))
            }
        }
        None
    }
}
impl Iterator for KingIterator {
    type Item = Square;
    fn next(&mut self) -> Option<Self::Item> {
        while self.index < Square::KING_OFFSETS.len() {
            let (row_offset, col_offset) = Square::KING_OFFSETS[self.index];
            self.index += 1;

            if let (Some(row), Some(col)) =
                (self.row + row_offset, self.col + col_offset) {
                    return Some(Square::new(row, col))
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
