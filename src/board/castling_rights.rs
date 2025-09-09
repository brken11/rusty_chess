//! Castling rights manager
//!
//! Module encodes and contains castling rights for compact `u8` bitmask.
//! Each of the four castling options (white kingside, white queenside,
//! black kingside, black queenside) take on bit.
//!
//! The [`CastlingRightsExt`] trait contains the methods for querying
//! and updating castling rights as the game progresses.

use super::*;
use crate::rules::CastleType;

/// Castling rights for both players, stored as bitmasks
///
/// - `0b0001` -> white kingside
/// - `0b0010` -> white queenside
/// - `0b0100` -> black kingside
/// - `0b1000` -> black queenside
pub type CastlingRights = u8;

/// Extension trait for method calls related to castling and castle rights
pub trait CastlingRightsExt{
    /// No player can castle
    const NONE_CAN_CASTLE: CastlingRights;
    /// All caslting options are available
    const ALL_CAN_CASTLE: CastlingRights;
    /// Mask covering kingside rights
    const KINGSIDE_MASK: CastlingRights;
    /// Mask covering queenside rights
    const QUEENSIDE_MASK: CastlingRights;
    /// Mask covering white's rights
    const WHITE_KING_MASK: CastlingRights;
    /// Mask covering black's rights
    const BLACK_KING_MASK: CastlingRights;

    /// Returns `true` if the given color can castle on the given side
    fn can_castle(self, color: Color, castle_type: CastleType) -> bool;
    /// Returns `true` if white can castle kingside
    fn can_castle_white_king_side(self) -> bool;
    /// Returns `true` if white can castle queenside
    fn can_castle_white_queen_side(self) -> bool;
    /// Returns `true` if black can castle kingside
    fn can_castle_black_king_side(self) -> bool;
    /// Returns `true` if black can castle queenside
    fn can_castle_black_queen_side(self) -> bool;

    /// Clears castling rights for the given king color
    fn king_moved(&mut self, king_color: Color);
    /// Clears the castling right for the player whom's rook has moved.
    ///
    /// # Arguments 
    ///
    /// - `rook_color`: Which color's rook has moved.
    /// - `kingside`: `true` if the rook was kingside, `false` if queenside.
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
    fn can_castle(self, color: Color, castle_type: CastleType) -> bool {
        let mask = match castle_type{
            CastleType::KingSide => CastlingRights::KINGSIDE_MASK,
            CastleType::QueenSide => CastlingRights::QUEENSIDE_MASK,
        };
        0 != match color{
            Color::White => {self & mask & CastlingRights::WHITE_KING_MASK}
            Color::Black => {self & mask & CastlingRights::BLACK_KING_MASK}
        }
    }
    #[inline]
    fn can_castle_white_king_side(self) -> bool {
        self & Self::WHITE_KING_MASK & Self::KINGSIDE_MASK != 0
    }
    #[inline]
    fn can_castle_white_queen_side(self) -> bool {
        self & Self::WHITE_KING_MASK & Self::QUEENSIDE_MASK != 0
    }
    #[inline]
    fn can_castle_black_king_side(self) -> bool {
        self & Self::BLACK_KING_MASK & Self::KINGSIDE_MASK != 0
    }
    #[inline]
    fn can_castle_black_queen_side(self) -> bool {
        self & Self::BLACK_KING_MASK & Self::QUEENSIDE_MASK != 0
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

/// Integration tests for castling right updates
#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that `can_castle()` correctly reposts rights
    /// for all rights and for non enabled.
    #[test]
    fn can_castle_output() {
        let all = CastlingRights::ALL_CAN_CASTLE;
        let none = CastlingRights::NONE_CAN_CASTLE;

        // All rights enabled, all should be true
        for color in Color::iter_colors() {
            assert!(all.can_castle(color, CastleType::KingSide));
            assert!(all.can_castle(color, CastleType::QueenSide));
        }

        // No rights enabled, every color/side should yield false
        for color in Color::iter_colors() {
            assert!(!none.can_castle(color, CastleType::KingSide));
            assert!(!none.can_castle(color, CastleType::QueenSide));
        }
    }
    /// Validates white kingside castling is correctly parsed.
    #[test]
    fn can_castle_white_king_side_output() {
        let all = CastlingRights::ALL_CAN_CASTLE;
        let none = CastlingRights::NONE_CAN_CASTLE;

        assert!(all.can_castle_white_king_side());
        assert!(!none.can_castle_white_king_side());
    }
    /// Validates white queenside castling is correctly parsed.
    #[test]
    fn can_castle_white_queen_side_output() {
        let all = CastlingRights::ALL_CAN_CASTLE;
        let none = CastlingRights::NONE_CAN_CASTLE;

        assert!(all.can_castle_white_queen_side());
        assert!(!none.can_castle_white_queen_side());
    }
    /// Validates black kingside castling is correctly parsed.
    #[test]
    fn can_castle_black_king_side_output() {
        let all = CastlingRights::ALL_CAN_CASTLE;
        let none = CastlingRights::NONE_CAN_CASTLE;

        assert!(all.can_castle_black_king_side());
        assert!(!none.can_castle_black_king_side());
    }
    /// Validates black queenside castling is correctly parsed.
    #[test]
    fn can_castle_black_queen_side_output() {
        let all = CastlingRights::ALL_CAN_CASTLE;
        let none = CastlingRights::NONE_CAN_CASTLE;

        assert!(all.can_castle_black_queen_side());
        assert!(!none.can_castle_black_queen_side());
    }

    /// Moving king should clear rights for only the color in question.
    #[test]
    fn king_moved_updates() {
        let mut castle_rights = CastlingRights::ALL_CAN_CASTLE;

        // White king moves
        castle_rights.king_moved(Color::White);

        // White king moved, no more castling
        assert!(!castle_rights.can_castle(Color::White, CastleType::KingSide));
        assert!(!castle_rights.can_castle(Color::White, CastleType::QueenSide));

        // Black's should remain unaffected
        assert!(castle_rights.can_castle(Color::Black, CastleType::KingSide));
        assert!(castle_rights.can_castle(Color::Black, CastleType::QueenSide));
    }

    /// Moving a rook should clear on the corresponding side of that color.
    #[test]
    fn rook_moved_updates() {
        let mut castle_rights = CastlingRights::ALL_CAN_CASTLE;

        // White kingside rook moved
        castle_rights.rook_moved(Color::White, true);

        // White kingside is no longer valid
        assert!(!castle_rights.can_castle(Color::White, CastleType::KingSide));
        // But Queenside is still valid
        assert!(castle_rights.can_castle(Color::White, CastleType::QueenSide));

        // Black remains unchanged
        assert!(castle_rights.can_castle(Color::Black, CastleType::KingSide));
        assert!(castle_rights.can_castle(Color::Black, CastleType::QueenSide));
    }
    #[test]
    fn game_sim() {
        let mut castle_rights = CastlingRights::ALL_CAN_CASTLE;

        // Black kingside rook moved
        castle_rights.rook_moved(Color::Black, true);

        // Black Kingside is no longer valid
        assert!(!castle_rights.can_castle(Color::Black, CastleType::KingSide));
        // Rest must be valid
        assert!(castle_rights.can_castle(Color::White, CastleType::KingSide));
        assert!(castle_rights.can_castle(Color::White, CastleType::QueenSide));
        assert!(castle_rights.can_castle(Color::Black, CastleType::QueenSide));

        // White king moved
        castle_rights.king_moved(Color::White);

        // White loses all rights
        assert!(!castle_rights.can_castle(Color::White, CastleType::KingSide));
        assert!(!castle_rights.can_castle(Color::White, CastleType::QueenSide));
        // Black kingside was already invalid
        assert!(!castle_rights.can_castle(Color::Black, CastleType::KingSide));
        // Black queenside still valid
        assert!(castle_rights.can_castle(Color::Black, CastleType::QueenSide));

        // Methods should be resistant redundant updates
        castle_rights.rook_moved(Color::Black, true);

        // State remains unchanged, still invalid
        assert!(!castle_rights.can_castle(Color::White, CastleType::KingSide));
        assert!(!castle_rights.can_castle(Color::White, CastleType::QueenSide));
        assert!(!castle_rights.can_castle(Color::Black, CastleType::KingSide));
        // State remains unchanged, still valid
        assert!(castle_rights.can_castle(Color::Black, CastleType::QueenSide));

        // Black king moves
        castle_rights.king_moved(Color::Black);

        // All castling rights have been used and revoked
        assert!(!castle_rights.can_castle(Color::White, CastleType::KingSide));
        assert!(!castle_rights.can_castle(Color::White, CastleType::QueenSide));
        assert!(!castle_rights.can_castle(Color::Black, CastleType::KingSide));
        assert!(!castle_rights.can_castle(Color::Black, CastleType::QueenSide));
    }
}
