
/// A square on the chessboard represented as a value between 0 and 63.
pub type Square = u8;
pub type Row = u8;
pub type Col = u8;

/// Provides extended functionality for chess board squares.
pub trait SquareExt {
    /// Size of Square bounds
    const MAX: Square;
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
impl SquareExt for Square {
    const MAX: u8 = Square::ROWS * Square::COLS;
    const ROWS: u8 = 8;
    const COLS: u8 = 8;
    const A8: Square =  0 as Square; const B8: Square =  1 as Square; const C8: Square =  2 as Square; const D8: Square =  3 as Square; const E8: Square =  4 as Square; const F8: Square =  5 as Square; const G8: Square =  6 as Square; const H8: Square =  7 as Square;
    const A7: Square =  8 as Square; const B7: Square =  9 as Square; const C7: Square = 10 as Square; const D7: Square = 11 as Square; const E7: Square = 12 as Square; const F7: Square = 13 as Square; const G7: Square = 14 as Square; const H7: Square = 15 as Square;
    const A6: Square = 16 as Square; const B6: Square = 17 as Square; const C6: Square = 18 as Square; const D6: Square = 19 as Square; const E6: Square = 20 as Square; const F6: Square = 21 as Square; const G6: Square = 22 as Square; const H6: Square = 23 as Square;
    const A5: Square = 24 as Square; const B5: Square = 25 as Square; const C5: Square = 26 as Square; const D5: Square = 27 as Square; const E5: Square = 28 as Square; const F5: Square = 29 as Square; const G5: Square = 30 as Square; const H5: Square = 31 as Square;
    const A4: Square = 32 as Square; const B4: Square = 33 as Square; const C4: Square = 34 as Square; const D4: Square = 35 as Square; const E4: Square = 36 as Square; const F4: Square = 37 as Square; const G4: Square = 38 as Square; const H4: Square = 39 as Square;
    const A3: Square = 40 as Square; const B3: Square = 41 as Square; const C3: Square = 42 as Square; const D3: Square = 43 as Square; const E3: Square = 44 as Square; const F3: Square = 45 as Square; const G3: Square = 46 as Square; const H3: Square = 47 as Square;
    const A2: Square = 48 as Square; const B2: Square = 49 as Square; const C2: Square = 50 as Square; const D2: Square = 51 as Square; const E2: Square = 52 as Square; const F2: Square = 53 as Square; const G2: Square = 54 as Square; const H2: Square = 55 as Square;
    const A1: Square = 56 as Square; const B1: Square = 57 as Square; const C1: Square = 58 as Square; const D1: Square = 59 as Square; const E1: Square = 60 as Square; const F1: Square = 61 as Square; const G1: Square = 62 as Square; const H1: Square = 63 as Square;
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
    fn get_row(&self) -> Row {
        self / Square::ROWS
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
        self % Square::COLS
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
    fn new(row : u8, col : u8) -> Square {
        row * Square::COLS + col
    }
    fn valid_new(row : u8, col : u8) -> Option<Square> {
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
    const MAX_ROWS: u8;
    ///
    ///
    /// # Arguments
    ///
    /// * `ascending`: whether you want the next value to be greater (if true) or lesser
    /// (if false) than the starting row.
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
}
pub trait ColExt {
    const MAX_COLS: u8;
    ///
    ///
    /// # Arguments
    ///
    /// * `ascending`: whether you want the next value to be greater (if true) or lesser
    /// (if false) than the starting row.
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
}
impl RowExt for Row {
    const MAX_ROWS: u8 = Square::ROWS;
    fn get_next_row(&self, ascending: bool) -> Option<Row> {
        let next: Row = if ascending { *self + 1 } else { *self - 1 };
        if next < Row::MAX_ROWS {
            Some(next)
        } else {
            None
        }
    }
    fn from_rank(rank: u8) -> Row {
        Row::MAX_ROWS - rank
    }
}
impl ColExt for Col {
    const MAX_COLS: u8 = Square::COLS;
    fn get_next_col(&self, ascending: bool) -> Option<Col> {
        let next: Col = if ascending { *self + 1 } else { *self - 1 };
        if next < Self::MAX_COLS {
            Some(next)
        } else {
            None
        }
    }
    fn from_file(file: char) -> Col {
        if file >= 'a' {
            file as u8 - 'a' as u8
        } else {
            file as u8 - 'A' as u8
        }
    }
}

pub struct DiagonalSquareIterator{
    row: Row,
    col: Col,
    ascending_row: bool,
    ascending_col: bool,
}
impl Iterator for DiagonalSquareIterator {
    type Item = Square;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_row) = self.row.get_next_row(self.ascending_row) {
            if let Some(next_col) = self.col.get_next_col(self.ascending_col) {
                self.row = next_row;
                self.col = next_col;
                Some(Square::new(next_row, next_col))
            } else {
                None
            }
        } else {
            None
        }
    }
}
pub struct SquareIterator{
    square: Square,
    next_square_offset: i8,
    squares_remaining : u8,
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
           None
       } else {
           self.square = self.square + self.next_square_offset as u8 ;
           self.squares_remaining -= 1;
           Some(self.square)
       }
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
            None => {None}
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
            None => {None}
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

// impl fmt::Display for Square {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         self.to_square_string().fmt(f)
//     }
// }




















