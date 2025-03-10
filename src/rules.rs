pub enum GameState {
    Running,
    Checkmate,
    Draw,
}

pub enum MoveResult {
    None,
    Check,
    Checkmate,
    Stalemate,
}

pub enum MoveType {
    Regular,
    Castling,
    Promotion,
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

pub enum CastleType {
    // None, // Yeah I don't know if this was actually needed.
    KingSide,
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