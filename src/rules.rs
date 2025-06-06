/// Describes the satus of the Chess game
pub enum GameState {
    /// The game is in progress
    Running,
    /// A player has been checkmated.
    Checkmate,
    /// The game has ended in a draw
    Draw,
}

#[derive(Debug)]
/// The outcome of a [crate::chess_moves:ChessMove] move
pub enum MoveResult {
    /// No special outcome
    None,
    /// The move puts the opponents king in **Check**
    Check,
    /// The move ends in a **Checkmate**
    Checkmate,
    /// The move ends with a **Stalemate**
    Stalemate,
}

#[derive(Debug)]
/// Enum of different types of chess moves
pub enum MoveType {
    /// A standard chess move
    Regular,
    /// A castling move, involving a [King](crate::board::pieces::Piece) and [Rook](crate::board::pieces::Piece)
    Castling,
    /// If a [Pawn](crate::board::pieces::Piece) move ends with a promotion
    Promotion,
    /// En passant capture
    EnPassant,
}

impl MoveResult {
    pub fn to_char(&self) -> Option<char> {
        match self {
            MoveResult::None => None,
            MoveResult::Check => Some('+'),
            MoveResult::Checkmate => Some('#'),
            MoveResult::Stalemate => Some('‡'),
        }
    }
}

#[derive(Debug,Eq, PartialEq, Clone, Copy)]
/// Indicates the direction of a castle
pub enum CastleType {
    // None, // Yeah I don't know if this was actually needed.
    /// King side (short) castle
    KingSide,
    /// Queen side (long) castle
    QueenSide,
}

impl CastleType {
    pub fn to_string(&self) ->  String{
        match self {
            // CastleType::None => String::from("INVALID_CASTLE_STATE"),
            CastleType::KingSide => String::from("O-O"),
            CastleType::QueenSide => String::from("O-O-O"),
        }
    }
}