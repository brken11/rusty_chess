use super::*;
use crate::rules::CastleType;

/// Contains castling rights for both white and black.
pub type CastlingRights = u8;
pub trait CastlingRightsExt{
    const NONE_CAN_CASTLE: CastlingRights;
    const ALL_CAN_CASTLE: CastlingRights;
    const KINGSIDE_MASK: CastlingRights;
    const QUEENSIDE_MASK: CastlingRights;
    const WHITE_KING_MASK: CastlingRights;
    const BLACK_KING_MASK: CastlingRights;
    fn can_castle(self, color: Color, castle_type: CastleType) -> bool;
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

#[cfg(test)]
mod tests {
    #[test]
    fn can_castle_output() {
        let all = CastlingRights::ALL_CAN_CASTLE;
        let none = CastlingRights::NONE_CAN_CASTLE;

        for color in Color::colo
    }
}
