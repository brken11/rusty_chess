use crate::board::pieces::Piece;
use crate::chess_moves::MoveError;

pub const PIECE_MAP: &[(&str, char)] = &[
    // Bishop
    ("B", 'B'), ("♗", 'B'), ("♝", 'B'), ("⒝", 'B'), ("ⓑ", 'B'), ("㋝", 'B'), ("🄑", 'B'), ("🅑", 'B'),
    ("✝️", 'B'), ("⚜️", 'B'),
    ("角", 'B'), ("象", 'B'), // Shogi & Xiangqi Bishops

    // King
    ("K", 'K'), ("♔", 'K'), ("♚", 'K'), ("⒦", 'K'), ("Ⓚ", 'K'), ("㋞", 'K'), ("ᴋ", 'K'), ("🄺", 'K'),
    ("🄚", 'K'), ("🅺", 'K'), ("🅚", 'K'), ("🤴", 'K'), ("👑", 'K'),
    ("王", 'K'), ("将", 'K'), ("帅", 'K'), // Shogi & Xiangqi Kings

    // Knight
    ("N", 'N'), ("♘", 'N'), ("♞", 'N'), ("⒩", 'N'), ("Ⓝ", 'N'), ("🄝", 'N'), ("🅝", 'N'), ("ᴎ", 'N'),
    ("🐴", 'N'), ("🏇", 'N'), ("🦄", 'N'),
    ("桂", 'N'), ("馬", 'N'), ("傌", 'N'), // Shogi & Xiangqi Knights

    // Pawn
    ("P", 'P'), ("♙", 'P'), ("♟", 'P'), ("⒫", 'P'), ("Ⓟ", 'P'), ("🄟", 'P'), ("🅟", 'P'),
    ("🏃", 'P'), ("🚶", 'P'),
    ("歩", 'P'), ("兵", 'P'), ("卒", 'P'), // Shogi & Xiangqi Pawns

    // Queen
    ("Q", 'Q'), ("♕", 'Q'), ("♛", 'Q'), ("⒬", 'Q'), ("Ⓠ", 'Q'), ("㋛", 'Q'), ("Q̲", 'Q'), ("🄌", 'Q'),
    ("🅀", 'Q'), ("👸", 'Q'), ("👑", 'Q'),

    // Rook
    ("R", 'R'), ("♖", 'R'), ("♜", 'R'), ("⒭", 'R'), ("Ⓡ", 'R'), ("㋜", 'R'), ("🄍", 'R'), ("🅁", 'R'),
    ("🏰", 'R'), ("🏯", 'R'),
    ("飛", 'R'), ("車", 'R'), ("俥", 'R'), // Shogi & Xiangqi Rooks
];
pub const SORTED_PIECE_MAP: &[(&str, char)] = &[("B", 'B'), ("K", 'K'), ("N", 'N'), ("P", 'P'), ("Q", 'Q'), ("Q̲", 'Q'), ("R", 'R'), ("ᴋ", 'K'), ("ᴎ", 'N'), ("⒝", 'B'), ("⒦", 'K'), ("⒩", 'N'), ("⒫", 'P'), ("⒬", 'Q'), ("⒭", 'R'), ("Ⓚ", 'K'), ("Ⓝ", 'N'), ("Ⓟ", 'P'), ("Ⓠ", 'Q'), ("Ⓡ", 'R'), ("ⓑ", 'B'), ("♔", 'K'), ("♕", 'Q'), ("♖", 'R'), ("♗", 'B'), ("♘", 'N'), ("♙", 'P'), ("♚", 'K'), ("♛", 'Q'), ("♜", 'R'), ("♝", 'B'), ("♞", 'N'), ("♟", 'P'), ("⚜️", 'B'), ("✝️", 'B'), ("㋛", 'Q'), ("㋜", 'R'), ("㋝", 'B'), ("㋞", 'K'), ("俥", 'R'), ("傌", 'N'), ("兵", 'P'), ("卒", 'P'), ("将", 'K'), ("帅", 'K'), ("桂", 'N'), ("歩", 'P'), ("王", 'K'), ("角", 'B'), ("象", 'B'), ("車", 'R'), ("飛", 'R'), ("馬", 'N'), ("🄌", 'Q'), ("🄍", 'R'), ("🄑", 'B'), ("🄚", 'K'), ("🄝", 'N'), ("🄟", 'P'), ("🄺", 'K'), ("🅀", 'Q'), ("🅁", 'R'), ("🅑", 'B'), ("🅚", 'K'), ("🅝", 'N'), ("🅟", 'P'), ("🅺", 'K'), ("🏃", 'P'), ("🏇", 'N'), ("🏯", 'R'), ("🏰", 'R'), ("🐴", 'N'), ("👑", 'K'), ("👑", 'Q'), ("👸", 'Q'), ("🚶", 'P'), ("🤴", 'K'), ("🦄", 'N'), ];
enum ParseError{
    IllegalMoveError(MoveError),
    DisambiguousMoveError,
    MissingTargetError,


}



pub(crate) mod chess_notation_parser {
    use crate::chess_moves::{ChessMove, MoveError};
    use crate::move_parser::{SORTED_PIECE_MAP, ParseError};

    fn from_simplified_chess_notation(string: String) -> Result<ChessMove, ParseError>{
        todo!("Implement me")
    }

    pub fn normalize_piece_symbol(symbol: &str) -> Option<char> {
        SORTED_PIECE_MAP.binary_search_by_key(&symbol, |&(key, _)| key)
            .ok()
            .map(|index| SORTED_PIECE_MAP[index].1)
    }
}

fn parse_piece(piece_slice: &str) -> Result<Piece, ParseError> {
    todo!("Implement me")
}
