use crate::board::pieces::{Color, Piece};
use crate::board::{Board, Square, SquareExt};
use crate::board::square::{Col, ColExt, Row, RowExt};
use crate::chess_moves::{ChessMove,MoveError,Disambiguity};
use crate::rules::{MoveResult, MoveType, CastleType};

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
pub const SORTED_PIECE_MAP: &[(char, char)] = &[('+', '+'), ('-', '-'), ('0', '0'), ('1', '1'), ('2', '2'), ('3', '3'), ('4', '4'), ('5', '5'), ('6', '6'), ('7', '7'), ('8', '8'), ('=', '='), ('>', '>'), ('B', 'B'), ('K', 'K'), ('N', 'N'), ('O', 'O'), ('P', 'P'), ('Q', 'Q'), ('R', 'R'), ('a', 'a'), ('b', 'b'), ('c', 'c'), ('d', 'd'), ('e', 'e'), ('f', 'f'), ('g', 'g'), ('h', 'h'), ('x', 'x'), ('ᴋ', 'K'), ('ᴎ', 'N'), ('⒝', 'B'), ('⒦', 'K'), ('⒩', 'N'), ('⒫', 'P'), ('⒬', 'Q'), ('⒭', 'R'), ('Ⓚ', 'K'), ('Ⓝ', 'N'), ('Ⓟ', 'P'), ('Ⓠ', 'Q'), ('Ⓡ', 'R'), ('ⓑ', 'B'), ('♔', 'K'), ('♕', 'Q'), ('♖', 'R'), ('♗', 'B'), ('♘', 'N'), ('♙', 'P'), ('♚', 'K'), ('♛', 'Q'), ('♜', 'R'), ('♝', 'B'), ('♞', 'N'), ('♟', 'P'), ('㋛', 'Q'), ('㋜', 'R'), ('㋝', 'B'), ('㋞', 'K'), ('俥', 'R'), ('傌', 'N'), ('兵', 'P'), ('卒', 'P'), ('将', 'K'), ('帅', 'K'), ('桂', 'N'), ('歩', 'P'), ('王', 'K'), ('角', 'B'), ('象', 'B'), ('車', 'R'), ('飛', 'R'), ('馬', 'N'), ('🄌', 'Q'), ('🄍', 'R'), ('🄑', 'B'), ('🄚', 'K'), ('🄝', 'N'), ('🄟', 'P'), ('🄺', 'K'), ('🅀', 'Q'), ('🅁', 'R'), ('🅑', 'B'), ('🅚', 'K'), ('🅝', 'N'), ('🅟', 'P'), ('🅺', 'K'), ('🏃', 'P'), ('🏇', 'N'), ('🏯', 'R'), ('🏰', 'R'), ('🐴', 'N'), ('👑', 'K'), ('👑', 'Q'), ('👸', 'Q'), ('🚶', 'P'), ('🤴', 'K'), ('🦄', 'N')];
#[derive(Debug)]
pub enum ParseError{
    MissingPiece,
    MissingOriginError,
    MissingTargetError,
    MissingPromotionPiece,
    MissingCastleType,
    InvalidSquare(Square),
    UnknownCharacter,
    UnrecognizedCharInMap,
    TokenizationError,
    AlgebraicParseModeError(AlgebraicParseStage),
    DisambiguousMoveError(Disambiguity),
    IllegalMoveError(MoveError),
    MalformedCastle,
    MalformedExpression(Token),
    Todo,
}

#[derive(Debug,Eq, PartialEq, Clone, Copy)]
pub enum Token {
    Piece(Piece),       // Pieces 'K', 'Q' etc
    Rank(Row),          // Ranks '1' through '8'
    File(Col),          // Files 'a' through 'h'
    Square(Square),     // 'a1', 'c3', 'h8' etc
    Capture,            // 'x'
    Separator,          // '-' and '>'
    Check,              // '+'
    Checkmate,          // '#'
    Stalemate,          // '‡'
    Promotion(Piece),   // '=N','=Q' etc
    Castle(CastleType),  // "O-O", "0-0-0" etc
}

#[derive(Debug,)]
pub struct ProtoMove {
    pub piece: Option<Piece>,
    pub origin: Disambiguity,
    pub is_capture: Option<bool>,
    pub target: Option<Square>,
    pub move_type: MoveType,
    pub promotion_piece: Option<Piece>,
    pub castle_type: Option<CastleType>,
    pub move_result: MoveResult,
}

#[derive(Debug,PartialEq, Eq, PartialOrd, Ord)]
pub enum AlgebraicParseStage{
    Piece,
    Disambiguation,
    Capture,
    Target,
    Promotion,
    MoveResult
}



pub(crate) mod chess_notation_parser {
    use super::*;

    pub fn from_simplified_algebraic_notation(string: &str, active_player: Color) -> Result<ProtoMove, ParseError>{
        let mut proto_move: ProtoMove = ProtoMove{
            piece : None,
            origin : Disambiguity::None,
            target : None,
            is_capture : None,
            move_type : MoveType::Regular,
            promotion_piece: None,
            castle_type: None,
            move_result : MoveResult::None,
        };
        let mut parse_step;// = AlgebraicParseStage::Piece;
        let tokens = tokenize_string(string, SORTED_PIECE_MAP, active_player)?;
        let tokens = pre_process_tokens(&tokens)?;

        let mut token_iter = tokens.into_iter().peekable();

        // Parse piece/castle
        match token_iter.peek() {
            None => return Err(ParseError::TokenizationError),
            Some(Token::Castle(castle_type)) => {
                proto_move.piece = Some(active_player.get_king());
                proto_move.move_type = MoveType::Castling;
                proto_move.castle_type = Some(*castle_type);

                token_iter.next();

                parse_step = AlgebraicParseStage::MoveResult;
            }
            Some(Token::Piece(piece)) => {
                proto_move.piece = Some(*piece);

                token_iter.next();

                parse_step = AlgebraicParseStage::Disambiguation;
            },
            Some(Token::File(col)) => {
                proto_move.piece = Some(active_player.get_pawn());
                proto_move.origin = Disambiguity::File(*col);
                
                token_iter.next();

                parse_step = AlgebraicParseStage::Capture;
            }
            Some(Token::Square(square)) => {
                proto_move.piece = Some(active_player.get_pawn());
                proto_move.target = Some(*square);

                token_iter.next();

                parse_step = AlgebraicParseStage::Capture;
            }
            Some(Token::Capture) => {
                proto_move.piece = Some(active_player.get_pawn());
                proto_move.origin = Disambiguity::None;//Redundant, but just to make it clear
                proto_move.is_capture = Some(true);

                token_iter.next();

                parse_step = AlgebraicParseStage::Target;
            }
            _ => return Err(ParseError::MissingPiece)
        }

        // Parse origin (if any)
        if parse_step <= AlgebraicParseStage::Disambiguation {match token_iter.peek() {
            None => return Err(ParseError::MissingTargetError),
            Some(Token::File(col)) => {
                proto_move.origin = Disambiguity::File(*col);

                token_iter.next();

                parse_step = AlgebraicParseStage::Capture;
            }
            Some(Token::Rank(row)) => {
                proto_move.origin = Disambiguity::Rank(*row);

                token_iter.next();

                parse_step = AlgebraicParseStage::Capture;
            }
            Some(Token::Square(square)) => {
                // Seems dumb to set this value to target, but if the only square is provided
                // using **Simplified Algebraic Notation**, it must be assumed it is the target
                // square and not the origin square, so until a second one shows up,
                // we assume it's the target.
                proto_move.target = Some(*square);

                token_iter.next();

                parse_step = AlgebraicParseStage::Capture;
            }
            Some(Token::Separator | Token::Capture) => {
                proto_move.origin = Disambiguity::None;
                parse_step = AlgebraicParseStage::Capture;
            }
            Some(Token::Piece(_) | Token::Check | Token::Checkmate | Token::Stalemate |
                Token::Promotion(_) | Token::Castle(_) ) => {
                return Err(ParseError::MalformedExpression(token_iter.next().unwrap()));
            }
        }}

        if parse_step <= AlgebraicParseStage::Capture { match token_iter.peek() {
            None => {
                return finalize(proto_move);
            }
            Some(Token::Capture) => {
                proto_move.is_capture = Some(true);

                token_iter.next();

                parse_step = AlgebraicParseStage::Target;
            }
            Some(Token::Separator) => {
                proto_move.is_capture = Some(false);

                token_iter.next();

                parse_step = AlgebraicParseStage::Target;
            }
            Some(Token::Square(_)) => {
                parse_step = AlgebraicParseStage::Target;
            }
            Some(Token::Promotion(_)) => {
                parse_step = AlgebraicParseStage::Promotion;
            }
            Some(Token::Check | Token::Checkmate | Token::Stalemate) => {
                parse_step = AlgebraicParseStage::MoveResult;
            }
            Some(Token::Piece(_) | Token::Rank(_) | Token::File(_) | Token::Castle(_)) => {
                return Err(ParseError::MalformedExpression(token_iter.next().unwrap()));
            }
        }}

        if parse_step <= AlgebraicParseStage::Target { match token_iter.peek() {
            None => {
                return finalize(proto_move);
            }
            Some(Token::Square(square)) => {
                match proto_move.target {
                    Some(old_square) => {
                        proto_move.origin = Disambiguity::Square(old_square);
                        proto_move.target= Some(*square);
                    }
                    None => proto_move.target = Some(*square)
                }

                token_iter.next();

                parse_step = AlgebraicParseStage::Promotion;
            }
            Some(Token::Promotion(_)) => {
                parse_step = AlgebraicParseStage::Promotion;
            }
            Some(Token::Check | Token::Checkmate | Token::Stalemate) => {
                parse_step = AlgebraicParseStage::MoveResult;
            }
            Some(Token::Piece(_) | Token::Rank(_) | Token::File(_) | Token::Castle(_) |
                    Token::Capture | Token::Separator) => {
                return Err(ParseError::MalformedExpression(token_iter.next().unwrap()));
            }
        }}

        if parse_step <= AlgebraicParseStage::Promotion { match token_iter.peek() {
            None => {
                return finalize(proto_move);
            }
            Some(Token::Promotion(piece)) => {
                proto_move.move_type = MoveType::Promotion;
                proto_move.promotion_piece = Some(*piece);

                token_iter.next();

                parse_step = AlgebraicParseStage::MoveResult;
            }
            Some(Token::Check | Token::Checkmate | Token::Stalemate) => {
                parse_step = AlgebraicParseStage::MoveResult;
            }
            Some(Token::Piece(_) | Token::Rank(_) | Token::File(_) | Token::Castle(_) |
                 Token::Capture | Token::Separator | Token::Square(_) ) => {
                return Err(ParseError::MalformedExpression(token_iter.next().unwrap()));
            }
        }}

        if parse_step > AlgebraicParseStage::MoveResult {
            return Err(ParseError::AlgebraicParseModeError(parse_step));
        }

        // Parse last token
        match token_iter.next() {
            None => return finalize(proto_move),
            Some(Token::Check) => {
                proto_move.move_result = MoveResult::Check;
            }
            Some(Token::Checkmate) => {
                proto_move.move_result = MoveResult::Checkmate;
            }
            Some(Token::Stalemate) => {
                proto_move.move_result = MoveResult::Stalemate;
            }
            Some(Token::Piece(_) | Token::Rank(_) | Token::File(_) | Token::Castle(_) |
                 Token::Capture | Token::Separator | Token::Square(_) | Token::Promotion(_)) => {
                return Err(ParseError::MalformedExpression(token_iter.next().unwrap()));
            }
        }

        // Token Vec should be emptied
        if token_iter.peek().is_some() {
            return Err(ParseError::MalformedExpression(token_iter.next().unwrap()));
        }

        finalize(proto_move)
    }
    pub fn from_long_algebraic_notation(string: &str, active_player: Color) -> Result<ProtoMove, ParseError> {
        let mut proto_move: ProtoMove = ProtoMove{
            piece : None,
            origin : Disambiguity::None,
            target : None,
            is_capture : None,
            move_type : MoveType::Regular,
            promotion_piece: None,
            castle_type: None,
            move_result : MoveResult::None,
        };
        let mut parse_step;// = AlgebraicParseStage::Piece;
        let tokens = tokenize_string(string, SORTED_PIECE_MAP, active_player)?;
        let tokens = pre_process_tokens(&tokens)?;

        let mut token_iter = tokens.into_iter().peekable();

        // Parse piece/castle
        match token_iter.peek() {
            None => return Err(ParseError::TokenizationError),
            Some(Token::Castle(castle_type)) => {
                proto_move.piece = Some(active_player.get_king());
                proto_move.move_type = MoveType::Castling;
                proto_move.castle_type = Some(*castle_type);

                token_iter.next();

                parse_step = AlgebraicParseStage::MoveResult;
            }
            Some(Token::Piece(piece)) => {
                proto_move.piece = Some(*piece);

                token_iter.next();

                parse_step = AlgebraicParseStage::Disambiguation;
            },
            Some(Token::Square(square)) => {
                proto_move.piece = Some(active_player.get_pawn());
                proto_move.origin = Disambiguity::Square(*square);

                token_iter.next();

                parse_step = AlgebraicParseStage::Capture;
            }
            Some(Token::Capture) => {
                return Err(ParseError::MissingOriginError);
            }
            _ => return Err(ParseError::MissingPiece)
        }

        if parse_step <= AlgebraicParseStage::Disambiguation { match token_iter.peek() {
            Some(Token::Square(square)) => {
                proto_move.origin = Disambiguity::Square(*square);

                token_iter.next();

                parse_step = AlgebraicParseStage::Capture;
            }
            None | Some(_) => return Err(ParseError::MissingOriginError),
        }}

        if parse_step <= AlgebraicParseStage::Capture { match token_iter.peek() {
            Some(Token::Capture) => {
                proto_move.is_capture = Some(true);

                token_iter.next();

                parse_step = AlgebraicParseStage::Target;
            }
            Some(Token::Separator) => {
                proto_move.is_capture = Some(false);

                token_iter.next();

                parse_step = AlgebraicParseStage::Target;
            }
            Some(Token::Square(square)) => {
                proto_move.target = Some(*square);

                token_iter.next();

                parse_step = AlgebraicParseStage::Promotion;
            }
            Some(token) => return Err(ParseError::MalformedExpression(*token)),
            None => return Err(ParseError::MissingTargetError)
        }}

        if parse_step <= AlgebraicParseStage::Target { match token_iter.peek() {
            Some(Token::Square(square)) => {
                proto_move.target = Some(*square);

                token_iter.next();

                parse_step = AlgebraicParseStage::Promotion;
            }
            None | Some(_) => return Err(ParseError::MissingTargetError),
        }}

        if parse_step <= AlgebraicParseStage::Promotion { match token_iter.peek() {
            None => return finalize(proto_move),
            Some(Token::Promotion(piece) | Token::Piece(piece)) => {
                proto_move.promotion_piece = Some(*piece);

                token_iter.next();

                parse_step = AlgebraicParseStage::MoveResult
            }
            Some(Token::Check | Token::Checkmate | Token::Stalemate) => {
                parse_step = AlgebraicParseStage::MoveResult;
            }
            Some(token) => return Err(ParseError::MalformedExpression(*token)),
        }}

        // Parse last token
        if parse_step <= AlgebraicParseStage::MoveResult { match token_iter.peek() {
            None => return finalize(proto_move),
            Some(Token::Check) => {
                proto_move.move_result = MoveResult::Check;
            }
            Some(Token::Checkmate) => {
                proto_move.move_result = MoveResult::Checkmate;
            }
            Some(Token::Stalemate) => {
                proto_move.move_result = MoveResult::Stalemate;
            }
            Some(token) => {
                return Err(ParseError::MalformedExpression(*token));
            }
        }}

        if parse_step > AlgebraicParseStage::MoveResult {
            return Err(ParseError::AlgebraicParseModeError(parse_step));
        }

        // Token Vec should be emptied
        if token_iter.peek().is_some() {
            return Err(ParseError::MalformedExpression(token_iter.next().unwrap()));
        }

        finalize(proto_move)
    }



    // pub fn normalize_piece_symbol(symbol: &str) -> Option<char> {
    //     PIECE_MAP.binary_search_by_key(&symbol, |&(key, _)| key)
    //         .ok()
    //         .map(|index| SORTED_PIECE_MAP[index].1)
    // }

    impl ChessMove {
        pub fn new_from_proto(board: &mut Board, proto_move: ProtoMove) -> Result<ChessMove, ParseError> {
            // Castle
            match proto_move.move_type {
                MoveType::Castling => {
                    return match proto_move.castle_type {
                        Some(castle_type) => { match ChessMove::new_castle(board, castle_type) {
                                Ok(chess_move) => Ok(chess_move),
                                Err(e) => Err(ParseError::IllegalMoveError(e))
                        }}
                        None => Err(ParseError::MissingCastleType)
                    }
                }
                MoveType::Regular | MoveType::Promotion | MoveType::EnPassant => {}
            }

            let target_square: Square;
            // Get rid of bad variants
            if proto_move.piece.is_none() && proto_move.origin == Disambiguity::None{
                return Err(ParseError::MissingPiece)
            }
            if let Some(target) = proto_move.target {
                target_square = target;
            } else {
                return Err(ParseError::MissingTargetError)
            }

            // If promotion, pass promotion piece
            if let Some(promotion_piece) = proto_move.promotion_piece {
                return match if let Disambiguity::Square(origin_square) = proto_move.origin {
                            ChessMove::valid_new(board, promotion_piece, origin_square, target_square, true)
                        } else {
                            ChessMove::new_with_disambiguation(board, promotion_piece, proto_move.origin, target_square, true)
                        } {
                    Ok(chess_move) => Ok(chess_move),
                    Err(e) => Err(ParseError::IllegalMoveError(e))
                };
            }

            let result: Result<ChessMove, MoveError> = match (proto_move.piece, proto_move.origin) {
                (None, Disambiguity::None) => return Err(ParseError::MissingPiece),
                (Some(piece), origin) => {
                    ChessMove::new_with_disambiguation(board, piece, origin, target_square, false)
                }
                (_, Disambiguity::Square(origin_square)) => {
                    ChessMove::new_from_squares(board, origin_square, target_square, false)
                }
                (None, _) => return Err(ParseError::MissingPiece)
            };
            match result {
                Ok(chess_move) => Ok(chess_move),
                Err(e) => Err(ParseError::IllegalMoveError(e))
            }
        }
    }


}

fn finalize(proto_move: ProtoMove) -> Result<ProtoMove, ParseError>{
    match proto_move.move_type {
        MoveType::Castling => {
            return Ok(proto_move);
        }
        _ => {}
    }
    if proto_move.target.is_some() {
        Ok(proto_move)
    } else {
        Err(ParseError::MissingTargetError)
    }
}

fn tokenize_string(string: &str, map:&[(char, char)], color: Color) -> Result<Vec<Token>, ParseError>{
    let mut tokens: Vec<Token> = Vec::with_capacity(string.len());
    let mut chars = string.chars().peekable();

    while let Some(c) = chars.next() {
        match map.binary_search_by_key(&c, |&(key, _)| key)
            .ok()
            .map(|index| map[index].1)
        {
            Some(c) => match c{
                //Pieces
                'K'=> tokens.push(Token::Piece(if color == Color::White {Piece::WhiteKing} else {Piece::BlackKing})),
                'Q' => tokens.push(Token::Piece(if color == Color::White {Piece::WhiteQueen} else {Piece::BlackQueen})),
                'B' => tokens.push(Token::Piece(if color == Color::White {Piece::WhiteBishop} else {Piece::BlackBishop})),
                'N' => tokens.push(Token::Piece(if color == Color::White {Piece::WhiteKnight} else {Piece::BlackKnight})),
                'R' => tokens.push(Token::Piece(if color == Color::White {Piece::WhiteRook} else {Piece::BlackRook})),
                'P' => tokens.push(Token::Piece(if color == Color::White {Piece::WhitePawn} else {Piece::BlackPawn})),
                // File
                'a'..='h' | 'A'..='H' => {
                    /*if let Some(next_char) = chars.next() { match next_char {
                    //File followed by Rank is a Square
                    '1'..='8' => {
                        chars.next(); // Consume '1'-'8'
                        tokens.push(
                            Token::Square(
                                Square::new(Row::from_rank(c as u8 - '0' as u8),Col::from_file(next_char))
                            )
                        );
                        continue;
                    }
                    //Just File, fall through and add File
                    _ => {}
                    }}*/
                    tokens.push(Token::File(Col::from_file(c)))
                },
                '1'..='8' => {tokens.push(Token::Rank(Row::from_rank(c as u8 - '0' as u8)))},
                // Capture
                'x' => tokens.push(Token::Capture),
                // Separator
                '-' | '>' => tokens.push(Token::Separator),
                // Check
                '+' => tokens.push(Token::Check),
                // Checkmate
                '#' => tokens.push(Token::Checkmate),
                // Stalemate
                '‡' => tokens.push(Token::Stalemate),
                // Promotion
                '=' => tokens.push(Token::Promotion(if color == Color::White {Piece::WhitePawn} else {Piece::BlackPawn})),
                // Castling
                'O' => {
                    // O-O
                    if chars.peek() == Some(&'-') { chars.next(); if chars.peek() == Some(&'O') {
                        chars.next();
                        // O-O-O
                        if chars.peek() == Some(&'-') { chars.next(); if chars.peek() == Some(&'O') {
                            chars.next();
                            tokens.push(Token::Castle(CastleType::QueenSide));
                        }} else {
                            tokens.push(Token::Castle(CastleType::KingSide));
                        }
                    }}
                }
                '0' => {
                    // 0-0
                    if chars.peek() == Some(&'-') { chars.next(); if chars.peek() == Some(&'0') {
                        chars.next();
                        // 0-0-0
                        if chars.peek() == Some(&'-') { chars.next(); if chars.peek() == Some(&'0') {
                            chars.next();
                            tokens.push(Token::Castle(CastleType::QueenSide));
                        }} else {
                            tokens.push(Token::Castle(CastleType::KingSide));
                        }
                    }}
                }
                _ => return Err(ParseError::UnrecognizedCharInMap)
            }
            _ => return Err(ParseError::UnknownCharacter)
        }

    }

    Ok(tokens)
}

fn pre_process_tokens(tokens: &Vec<Token>) -> Result<Vec<Token>, ParseError>{
    let mut processed_tokens: Vec<Token> = Vec::with_capacity(tokens.len());
    let mut token_iter = tokens.into_iter().peekable();

    while let Some(token) = token_iter.next() { match token {
            Token::Promotion(_) => {
                match token_iter.peek() {
                    Some(Token::Piece(next_piece)) => {
                        token_iter.next(); // Consume next Token
                        processed_tokens.push(Token::Promotion(*next_piece));
                    }
                    _ => return Err(ParseError::MissingPromotionPiece)
                }
            }
            Token::File(col) => {
                if let Some(Token::Rank(row)) = token_iter.peek() {
                    if let Some(square) = Square::valid_new(*row, *col) {
                        token_iter.next(); //Consume next Token
                        processed_tokens.push(Token::Square(square));
                    } else {
                        return Err(ParseError::InvalidSquare(Square::new(*row, *col)));
                    }
                } else {
                    processed_tokens.push(Token::File(*col));
                }
            }
            // Token::Square(square) => {
            //     processed_tokens.push(Token::Square(*square));
            //     if let Some(Token::Piece(next_piece)) = token_iter.peek() {
            //         token_iter.next(); //Consume next Token
            //         processed_tokens.push(Token::Promotion(*next_piece));
            //     }
            // }
            _ => processed_tokens.push(token.clone()),
    }}

    Ok(processed_tokens)
}

/*fn analyze_token_vec(tokens: Vec<Token>) -> () {
    let mut square_count = 0;
    let mut piece_count = 0;
    let mut file_count = 0;
    let mut rank_count = 0;
    let mut flag_count = 0;
    let mut is_capture = false;
    let mut is_promotion = false;

    for token in tokens {
        match token {
            Token::Square(s) => square_count+= 1,
            Token::Piece(p) => piece_count+= 1,
            Token::File(f) => file_count += 1,
            Token::Rank(r) => rank_count += 1,
            Token::Check | Token::Checkmate | Token::Stalemate => flag_count += 1,
            Token::Capture => is_capture = true,
            Token::Promotion(p) => is_promotion = true,
            _ => {}
        }
    }
}*/

// fn normalize_piece(piece_slice: &mut str, piz) -> Result<Piece, ParseError> {
//
// }

#[cfg(test)]
mod tests {
    use super::*;
    use super::chess_notation_parser::*;

    const TEST_MAP: &[(char, char)] = &[
        ('#', '#'),('+', '+'),('-', '-'),('0', '0'),('1', '1'),('2', '2'),('3', '3'),('4', '4'),('5', '5'),('6', '6'),('7', '7'),('8', '8'),('=', '='),('B', 'B'),('K', 'K'),('N', 'N'),('O', 'O'),('P', 'P'),('Q', 'Q'),('R', 'R'),('a', 'a'),('b', 'b'),('c', 'c'),('d', 'd'),('e', 'e'),('f', 'f'),('g', 'g'),('h', 'h'),('x', 'x'),('‡', '‡'),
    ];

    #[test]
    fn test_tokenize_piece() {
        // Test that a single piece symbol is recognized.
        let tokens = tokenize_string("K", TEST_MAP, Color::White).unwrap();
        assert_eq!(tokens, vec![Token::Piece(Piece::WhiteKing)]);
    }

    #[test]
    fn test_tokenize_file_and_rank() {
        // The commented-out code in tokenize_string indicates that file and rank tokens are separate.
        // For example, "a1" should produce a File token then a Rank token.
        let tokens = tokenize_string("a1", TEST_MAP, Color::White).unwrap();
        // We expect:
        //   Token::File(Col::from_file('a'))
        //   Token::Rank(Row::from_rank(1))
        assert_eq!(tokens, vec![
            Token::File(Col::from_file('a')),
            Token::Rank(Row::from_rank(1))
        ]);
    }

    #[test]
    fn test_tokenize_capture_and_check() {
        // Test capture 'x' and check '+' tokens.
        let tokens = tokenize_string("x+", TEST_MAP, Color::Black).unwrap();
        assert_eq!(tokens, vec![
            Token::Capture,
            Token::Check,
        ]);
    }

    #[test]
    fn test_tokenize_promotion() {
        // Test promotion token '=' should produce a Promotion token using the pawn of the given color.
        let tokens = tokenize_string("=", TEST_MAP, Color::White).unwrap();
        assert_eq!(tokens, vec![
            Token::Promotion(Piece::WhitePawn)
        ]);
    }

    #[test]
    fn test_tokenize_kingside_castling_o() {
        // Test kingside castling using letter "O"
        let tokens = tokenize_string("O-O", TEST_MAP, Color::Black).unwrap();
        assert_eq!(tokens, vec![
            Token::Castle(CastleType::KingSide)
        ]);
    }

    #[test]
    fn test_tokenize_queenside_castling_o() {
        // Test queenside castling using letter "O"
        let tokens = tokenize_string("O-O-O", TEST_MAP, Color::White).unwrap();
        assert_eq!(tokens, vec![
            Token::Castle(CastleType::QueenSide)
        ]);
    }

    #[test]
    fn test_tokenize_kingside_castling_zero() {
        // Test kingside castling using zero "0"
        let tokens = tokenize_string("0-0", TEST_MAP, Color::White).unwrap();
        assert_eq!(tokens, vec![
            Token::Castle(CastleType::KingSide)
        ]);
    }

    #[test]
    fn test_tokenize_queenside_castling_zero() {
        // Test queenside castling using zero "0"
        let tokens = tokenize_string("0-0-0", TEST_MAP, Color::Black).unwrap();
        assert_eq!(tokens, vec![
            Token::Castle(CastleType::QueenSide)
        ]);
    }

    #[test]
    fn test_mixed_tokens() {
        // A more complex test that mixes piece, file, rank, capture, and check tokens.
        // Example input: "Nf3xg5+"
        // The function as written does not combine file+rank into Square tokens, so
        // it should output separate tokens for each character.
        let tokens = tokenize_string("Nf3xg5+", TEST_MAP, Color::White).unwrap();
        // Expected tokens:
        // 'N' -> Piece: WhiteKnight
        // 'f' -> File token (file f)
        // '3' -> Rank token (rank 3)
        // 'x' -> Capture
        // 'g' -> File token (file g)
        // '5' -> Rank token (rank 5)
        // '+' -> Check
        assert_eq!(tokens, vec![
            Token::Piece(Piece::WhiteKnight),
            Token::File(Col::from_file('f')),
            Token::Rank(Row::from_rank(3)),
            Token::Capture,
            Token::File(Col::from_file('g')),
            Token::Rank(Row::from_rank(5)),
            Token::Check,
        ]);
    }

    #[test]
    fn test_unrecognized_char() {
        // Test that an unrecognized character like 'z' triggers an UnrecognizedCharInMap error.
        let result = tokenize_string("z", TEST_MAP, Color::White);
        assert!(matches!(result, Err(ParseError::UnknownCharacter)));
    }

    #[test]
    fn test_promotion_pre_processing() {
        let raw = tokenize_string("=Q", TEST_MAP, Color::White).unwrap();
        let processed = pre_process_tokens(&raw).unwrap();
        assert_eq!(processed, vec![Token::Promotion(Piece::WhiteQueen)]);
    }

    #[test]
    fn test_square_pre_processing() {
        let raw = tokenize_string("a1", TEST_MAP, Color::White).unwrap();
        let processed = pre_process_tokens(&raw).unwrap();
        assert_eq!(
            processed,
            vec![Token::Square(Square::new(Row::from_rank(1), Col::from_file('a')))]
        );
    }

    #[test]
    fn test_san_simple_knight_move() {
        let result = from_simplified_algebraic_notation("Nf3", Color::White);
        assert!(matches!(result, Ok(_)));
    }

    #[test]
    fn test_san_disambiguation_by_file() {
        let result = from_simplified_algebraic_notation("Nbd2", Color::White);
        assert!(matches!(result, Ok(_)));
    }

    #[test]
    fn test_san_capture_with_check() {
        let result = from_simplified_algebraic_notation("Qxe5+", Color::White);
        assert!(matches!(result, Ok(_)));
    }

    #[test]
    fn test_san_promotion() {
        let result = from_simplified_algebraic_notation("e8=Q", Color::White);
        assert!(matches!(result, Ok(_)));
    }

    #[test]
    fn test_san_kingside_castle() {
        let result = from_simplified_algebraic_notation("O-O", Color::White);
        assert!(matches!(result, Ok(_)));
    }

    #[test]
    fn test_san_invalid_token() {
        let result = from_simplified_algebraic_notation("Nb@", Color::White);
        assert!(matches!(result, Err(ParseError::UnknownCharacter)));
    }

    #[test]
    fn test_lan_basic_move() {
        let result = from_long_algebraic_notation("e2-e4", Color::White);
        assert!(matches!(result, Ok(_)));
    }

    #[test]
    fn test_lan_promotion() {
        let result = from_long_algebraic_notation("e7-e8Q", Color::White);
        let result2 = from_long_algebraic_notation("a2xb1=N", Color::Black);
        assert!(matches!(result, Ok(_)));
        assert!(matches!(result2, Ok(_)))
    }

    #[test]
    fn test_lan_missing_origin() {
        let result = from_long_algebraic_notation("xe4", Color::White);
        assert!(matches!(result, Err(ParseError::MissingOriginError)));
    }

    #[test]
    fn test_proto_missing_target_square() {
        let proto = ProtoMove {
            piece: Some(Piece::WhiteQueen),
            origin: Disambiguity::File(Col::from_file('d')),
            is_capture: Some(true),
            target: None,
            move_type: MoveType::Regular,
            promotion_piece: None,
            castle_type: None,
            move_result: MoveResult::Check,
        };
        let mut board = Board::std_new();
        let result = ChessMove::new_from_proto(&mut board, proto);
        assert!(matches!(result, Err(ParseError::MissingTargetError)));
    }

    #[test]
    fn test_chess_move_knight_opening() {
        let mut board = Board::std_new(); // Standard board
        let proto = from_simplified_algebraic_notation("Nf3", Color::White).unwrap();
        let mv = ChessMove::new_from_proto(&mut board, proto);
        assert!(mv.is_ok());
    }

    #[test]
    fn test_chess_move_pawn_two_step() {
        let mut board = Board::std_new();
        let proto = from_simplified_algebraic_notation("e4", Color::White).unwrap();
        let mv = ChessMove::new_from_proto(&mut board, proto);
        assert!(mv.is_ok());
    }

    #[test]
    fn test_chess_move_disambiguate() {
        let mut board = Board::std_new();
        ChessMove::new_from_squares(&mut board, Square::E2, Square::E4, false).unwrap().make_move(&mut board); // e4
        ChessMove::new_from_squares(&mut board, Square::D7, Square::D5, false).unwrap().make_move(&mut board); // d5
        ChessMove::new_from_squares(&mut board, Square::B1, Square::C3, false).unwrap().make_move(&mut board); // Nc3
        ChessMove::new_from_squares(&mut board, Square::F7, Square::F5, false).unwrap().make_move(&mut board); // f5

        let proto1 = from_simplified_algebraic_notation("N3e2", Color::White);
        assert!(proto1.is_ok());
        let mv1 = ChessMove::new_from_proto(&mut board, proto1.unwrap()); // Two knights can go to e2
        assert!(mv1.is_ok());
        mv1.unwrap().make_move(&mut board);

        let proto2 = from_simplified_algebraic_notation("Pxe4", Color::Black); // Two pawns can go to e4
        assert!(proto2.is_ok());// Parse goes normally
        let mv2 = ChessMove::new_from_proto(&mut board, proto2.unwrap()); // Returns IllegalMoveError(DisambiguousMove)
        assert!(mv2.is_err());
        let proto3 = from_simplified_algebraic_notation("dxe4", Color::Black);
        assert!(proto3.is_ok());
        let mv3 = ChessMove::new_from_proto(&mut board, proto3.unwrap());
        mv3.unwrap().make_move(&mut board);
        assert!(true);
    }

    #[test]
    fn test_chess_move_illegal_rook_jump() {
        let mut board = Board::std_new();
        let proto = from_simplified_algebraic_notation("Ra3", Color::White).unwrap(); // Rook can't jump
        let mv = ChessMove::new_from_proto(&mut board, proto);
        assert!(matches!(mv, Err(ParseError::IllegalMoveError(_))));
    }
}
















