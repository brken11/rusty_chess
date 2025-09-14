use super::*;
/// A bitboard is used to represent piece positions using a 64-bit number.
pub type Bitboard = u64;
pub trait BitboardExt{
    fn new_bitboard() -> Self;
    /// # Arguments
    /// * row - an instance of `Row` that can be converted into a bitmask
    ///   for bulk comparison
    ///
    /// # Returns
    /// A `Bitboard` with the corresponding matching board squares represented
    /// as 1s on the mask
    fn get_bitboard_from_row(row: Row) -> Self;
    /// # Arguments
    /// * col - an instance of `Col` that can be converted into a bitmask
    ///   for bulk comparison
    ///
    /// # Returns
    /// A `Bitboard` with the corresponding matching board squares represented
    /// as 1s on the mask
    fn get_bitboard_from_col(col: Col) -> Self;
    /// A function that returns whether the corresponding square is in the
    /// bitmask or not
    ///
    /// # Returns
    /// - true: if the square is in the mask
    /// - false: if the square is ***not*** in the mask
    fn square_in_bitboard(self, square: Square) -> bool;
    /// The inverse of `square_in_bitboard`. Returns wether the corresponding
    /// square is ***not*** in the bitmask.
    /// # Returns
    /// - true: if the square is ***not*** in the mask
    /// - false: if the square ***is*** in the mask
    fn not_in_bitboard(self, square: Square) -> bool;
    /// Checks if the only bit in the bitboard mask is the given square.
    ///
    /// # Returns
    /// - true: if the square is the ***only*** square in the mask
    /// - false: if the square
    fn bitboard_is_square(self, square: Square) -> bool;
    fn squares(self) -> BitboardIter;
    fn remove_square(&mut self, square: Square);
    fn add_square(&mut self, square: Square);
}
impl BitboardExt for Bitboard {
    fn new_bitboard() -> Bitboard {
        0
    }
    fn get_bitboard_from_row(row: Row) -> Self {
        0x00000000000000FF << (row * Square::COLS)
    }
    fn get_bitboard_from_col(col:Col) -> Self {
        0x0101010101010101 << col
    }
    fn square_in_bitboard(self, square: Square) -> bool {
        (self & (1 << square)) != 0
    }
    fn not_in_bitboard(self, square: Square) -> bool {
        self & (1 << square) == 0
    }
    fn bitboard_is_square(self, square: Square) -> bool {
        self == (1 << square)
    }
    fn squares(self) -> BitboardIter {
        BitboardIter{bitboard: self}
    }
    fn remove_square(&mut self, square: Square) {
        *self &= ! (1 << square)
    }
    fn add_square(&mut self, square: Square) {
        *self |= 1 << square
    }
}

pub struct BitboardIter {
    bitboard: Bitboard
}
impl Iterator for BitboardIter {
    type Item = Square;
    fn next(&mut self) -> Option<Self::Item> {
        if self.bitboard == 0 {return None}

        let next_square: Square = self.bitboard.trailing_zeros() as Square;
        self.bitboard &= !(1 << next_square);

        Some(next_square)
    }
}

#[cfg(test)]
mod bitboard_tests {
    use super::*;

    #[test]
    fn test_new_bitboard_is_empty() {
        let bb = Bitboard::new_bitboard();
        assert_eq!(bb.count_ones(), 0);
    }

    #[test]
    fn test_add_and_remove_square() {
        let mut bb = Bitboard::new_bitboard();
        let sq = 36; // e4
        bb.add_square(sq);
        assert!(bb.square_in_bitboard(sq));
        assert_eq!(bb.count_ones(), 1);

        bb.remove_square(sq);
        assert!(!bb.square_in_bitboard(sq));
        assert_eq!(bb.count_ones(), 0);
    }

    #[test]
    fn test_not_in_bitboard() {
        let mut bb = Bitboard::new_bitboard();
        let sq = 10;
        assert!(bb.not_in_bitboard(sq));
        bb.add_square(sq);
        assert!(!bb.not_in_bitboard(sq));
    }

    #[test]
    fn test_bitboard_iteration() {
        let mut bb = Bitboard::new_bitboard();
        bb.add_square(0);
        bb.add_square(7);
        bb.add_square(63);

        let squares: Vec<_> = bb.squares().collect();
        assert_eq!(squares, vec![0, 7, 63]);
    }

    #[test]
    fn test_bitwise_ops() {
        let mut a = Bitboard::new_bitboard();
        let mut b = Bitboard::new_bitboard();
        a.add_square(0);
        b.add_square(0);
        b.add_square(1);

        let and_bb = a & b;
        assert!(and_bb.square_in_bitboard(0));
        assert!(!and_bb.square_in_bitboard(1));

        let or_bb = a | b;
        assert!(or_bb.square_in_bitboard(0));
        assert!(or_bb.square_in_bitboard(1));

        let xor_bb = a ^ b;
        assert!(!xor_bb.square_in_bitboard(0));
        assert!(xor_bb.square_in_bitboard(1));
    }
}
