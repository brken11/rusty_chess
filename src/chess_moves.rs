use std::fmt;
use std::fmt::Display;
use crate::rules;
use rules::MoveResult;
use rules::CastleType;

use crate::board::{pieces, Square, SquareExt, Board};
use pieces::Piece;
use crate::board::pieces::Color;
use crate::rules::MoveType;

#[repr(u8)]
#[derive(Debug)]
pub enum MoveData {
    Normal = 0,
    NormalCheck = 1,
    NormalCheckmate = 2,
    NormalStalemate = 3,
    Castling = 4,
    CastlingCheck = 5,
    CastlingCheckmate = 6,
    CastlingStalemate = 7,
    Promotion = 8,
    PromotionCheck = 9,
    PromotionCheckmate = 10,
    PromotionStalemate = 11,
    EnPassant = 12,
    EnPassantCheck = 13,
    EnPassantCheckmate = 14,
    EnPassantStalemate = 15,
    EnableEnPassant = 16,
    EnableEnPassantCheck = 17,
    EnableEnPassantCheckmate = 18,
    EnableEnPassantStalemate = 19,
    Capture = 20,
    CaptureCheck = 21,
    CaptureCheckmate = 22,
    CaptureStalemate = 23,
    CapturePromotion = 24,
    CapturePromotionCheck = 25,
    CapturePromotionCheckmate = 26,
    CapturePromotionStalemate = 27,
    //Moves may be generated by the engine ahead of time, so they may need to be properly classified.
    GeneratedOnly,
}

pub enum SpecialMoveType{
    None,//Has Capture Variant
    Castling,
    Promotion,//Has capture Variant
    EnPassant,//Implicitly captures
    EnableEnPassant,
}

pub enum MoveError{
    CapturedPieceNotProvided,
    LeavesKingInCheck,
    PieceNotFound,
    ObstructedMove
}

#[derive(Debug,)]
pub struct ChessMove {
    piece: Piece,
    origin: Square,
    target: Square,
    meta_data: MoveData,
}

impl ChessMove {
    fn new(piece: Piece, origin: Square, target: Square, special: MoveData) -> ChessMove {
        ChessMove {
            piece,
            origin,
            target,
            meta_data: special,
        }
    }
    pub fn assemble_special_move(move_type: MoveType, captures: bool, enables_en_passant: bool, result: MoveResult) -> MoveData {
        if enables_en_passant {
            build_move_type(MoveData::EnableEnPassant, result)
        } else {
            match move_type {
                MoveType::Regular => build_none_or_promotion(MoveData::Normal, captures, MoveData::Capture, result),
                MoveType::Castling => build_move_type(MoveData::Castling, result),
                MoveType::EnPassant => build_move_type(MoveData::EnPassant, result),
                MoveType::Promotion => build_none_or_promotion(MoveData::Promotion, captures, MoveData::CapturePromotion, result),
            }
        }
    }

    pub fn to_long_algebraic(&self) -> String {
        let is_promotion = self.meta_data.is_promotion();
        let initial_char:Option<&str> = if is_promotion {None} else {self.piece.to_str()};

        let mut algebraic_notation = if self.meta_data.is_castle() {
            if self.origin.get_col() > self.target.get_col() {
                CastleType::KingSide.to_string()
            } else {
                CastleType::QueenSide.to_string()
            }
        } else {
            format!(
                "{}{}{}{}",
                if initial_char.is_none() { "" } else { initial_char.unwrap() },
                self.origin.to_square_string(),
                if self.meta_data.is_capture() { "x" } else { "-" },
                self.target.to_square_string()
            )
        };

        if is_promotion {
            algebraic_notation.push('=');
            algebraic_notation.push(self.piece.to_char());
        }
        if let Some(move_result) = self.meta_data.move_result().to_char() {
            algebraic_notation.push(move_result)
        }
        algebraic_notation
    }
    pub fn to_simplified(&self) -> String {
        //@TODO give own logic
        self.to_long_algebraic()
    }

    fn make_move_on_board(&self, board: &mut Board){
        match self.meta_data.get_move_type() {
            MoveType::Regular => {
                if self.meta_data.is_capture() {
                    if let Some(target_piece) = board.get_piece_at(self.target) {
                        _ = board.remove_piece_at(self.target, target_piece)
                    }
                }
                _ = board.remove_piece_at(self.origin, self.piece);
                _ = board.add_piece_at(self.target, self.piece);

                if self.meta_data.is_enable_en_passant() {
                    board.en_passant_square = Some(self.origin)
                } else {
                    board.en_passant_square = None;
                }
            }
            MoveType::Castling => {
                let color = self.piece.get_color();
                let rook = color.get_rook();
                let rook_row = color.get_back_rank_row();
                _ = board.remove_piece_at(self.origin, self.piece);
                _ = board.add_piece_at(self.target, self.piece);
                let rook_col_old: u8;
                let rook_col_new: u8;
                (rook_col_old, rook_col_new) = if self.target > self.origin { // King side castle
                    (7, 5)
                } else { // Queen side castle
                    (0, 3)
                };
                let rook_square = Square::new(rook_row, rook_col_old);
                _ = board.remove_piece_at(rook_square, rook);
                let rook_square = Square::new(rook_row, rook_col_new);
                _ = board.add_piece_at(rook_square, rook);
            }
            MoveType::EnPassant => {
                let removal_square = Square::new(self.origin.get_row(), self.target.get_col());
                let target_pawn = match self.piece.get_color() {
                    Color::White => Piece::BlackPawn,
                    Color::Black => Piece::WhitePawn,
                };
                _ = board.remove_piece_at(self.origin, self.piece);
                _ = board.add_piece_at(self.target, self.piece);
                _ = board.remove_piece_at(removal_square, target_pawn);
            }
            MoveType::Promotion => {
                if self.meta_data.is_capture() {
                    if let Some(target_piece) = board.get_piece_at(self.target) {
                        _ = board.remove_piece_at(self.target, target_piece)
                    }
                }
                let pawn = match self.piece.get_color() {
                    Color::White => Piece::WhitePawn,
                    Color::Black => Piece::BlackPawn,
                };
                _ = board.remove_piece_at(self.origin, pawn);
                _ = board.add_piece_at(self.target, self.piece);
            }
        }
        board.active_player = board.active_player.toggle_color();
        match board.active_player {
            Color::White => {
                board.full_move_number += 1;
            }
            Color::Black => {},
        }
        if self.meta_data.reset_halfmove() || self.piece.is_pawn() {
            board.half_move_clock = 0;
        }
    }
    pub fn make_move(&self, board: &mut Board) {
        self.make_move_on_board(board);
    }
    fn make_reversible_move(&self, board: &mut Board) -> (Option<Piece>, Option<Square>, u8) {
        let old_en_passant_square = board.en_passant_square;
        let old_half_move_clock = board.half_move_clock;

        let removed_piece = if self.meta_data.is_capture() {
            match self.meta_data.get_move_type() {
                MoveType::Regular | MoveType::Promotion => board.get_piece_at(self.target),
                MoveType::EnPassant => board.get_piece_at(
                    Square::new(self.origin.get_row(), self.target.get_col())
                ),
                _ => !unreachable!()
            }
        } else {
            None
        };

        self.make_move_on_board(board);

        (removed_piece, old_en_passant_square, old_half_move_clock)
    }

    fn undo_move(&self, mut board: &mut Board, removed_piece: Option<Piece>, old_en_passant_square: Option<Square>, old_half_move_clock: u8) -> Result<(),MoveError>{
        board.active_player = board.active_player.toggle_color();
        match board.active_player {
            Color::White => {},
            Color::Black => {
                board.full_move_number -= 1;
            }
        }
        board.en_passant_square = old_en_passant_square;
        board.half_move_clock = old_half_move_clock;

        match self.meta_data.get_move_type() {
            MoveType::Regular => {
                let _ = board.remove_piece_at(self.target, self.piece);
                let _ = board.add_piece_at(self.origin, self.piece);
                if self.meta_data.is_capture() {
                    let _ = board.add_piece_at(self.target, removed_piece.unwrap());
                }
            }
            MoveType::Promotion => {
                let _ = board.remove_piece_at(self.target, self.piece);
                let _ = board.add_piece_at(self.origin, board.active_player.get_pawn());
                if self.meta_data.is_capture() {
                    let _ = board.add_piece_at(self.target, removed_piece.unwrap());
                }
            }
            MoveType::EnPassant => {
                let _ = board.remove_piece_at(self.target, self.piece);
                let _ = board.add_piece_at(self.origin, self.piece);
                let _ = board.add_piece_at(Square::new(self.origin.get_row(), self.target.get_col()), removed_piece.unwrap());
            }
            MoveType::Castling => {
                let color = self.piece.get_color();
                let rook = color.get_rook();
                let rook_row = color.get_back_rank_row();

                //Undo king move
                _ = board.remove_piece_at(self.origin, self.piece);
                _ = board.add_piece_at(self.target, self.piece);

                //Undo rook move
                let rook_col_old: u8;
                let rook_col_new: u8;
                (rook_col_old, rook_col_new) = if self.target > self.origin { // King side castle
                    (7, 5)
                } else { // Queen side castle
                    (0, 3)
                };
                let rook_square = Square::new(rook_row, rook_col_new);
                _ = board.remove_piece_at(rook_square, rook);
                let rook_square = Square::new(rook_row, rook_col_old);
                _ = board.add_piece_at(rook_square, rook);
            }
        }

        Ok(())
    }
    
    pub fn leaves_king_in_check(&self, board: &mut Board) -> bool {
        let (
            removed_piece,
            old_en_passant_range,
            old_half_move_clock) = self.make_reversible_move(board);

        //TODO- Add code to check if king is in check

        let _ = self.undo_move(board, removed_piece, old_en_passant_range, old_half_move_clock);

        false
    }
    pub fn get_valid_moves(board: &mut Board) -> Vec<ChessMove> {
        let possible_moves: Vec<ChessMove> = Self::get_possible_moves(board);
        Self::validate_moves(possible_moves, board)
    }
    fn validate_moves(moves: Vec<ChessMove>, board: &mut Board) -> Vec<ChessMove> {
        moves.into_iter()
            .filter(|m| !m.leaves_king_in_check(board))
            .collect()
    }
    fn get_possible_moves(board: &Board) -> Vec<ChessMove> {
        let mut moves: Vec<ChessMove> = Vec::with_capacity(64);
        for piece in Piece::iter_color_pieces(&board.active_player){
            match piece {
                Piece::WhitePawn | Piece::BlackPawn=> ChessMove::add_pawn_moves(&board, &mut moves),
                Piece::WhiteRook | Piece::BlackRook=> ChessMove::add_rook_moves(&board, &mut moves),
                Piece::WhiteKnight | Piece::BlackKnight=> ChessMove::add_knight_moves(&board, &mut moves),
                Piece::WhiteBishop | Piece::BlackBishop=> ChessMove::add_bishop_moves(&board, &mut moves),
                Piece::WhiteQueen | Piece::BlackQueen=> ChessMove::add_queen_moves(&board, &mut moves),
                Piece::WhiteKing | Piece::BlackKing=> ChessMove::add_king_moves(&board, &mut moves),
            }
        }
        moves
    }
    fn add_pawn_moves(board: &Board, moves: &mut Vec<ChessMove>) {
        let active_player = board.active_player;
        let pawn = active_player.get_pawn();
        let pawn_row_ascending = active_player.is_pawn_ascending();
        let starting_row = active_player.get_pawn_starting_row();
        let promotion_row = active_player.get_pawn_promotion_row();
        let mut bitboard = board.get_bitboard(pawn);

        while bitboard != 0 {
            let origin_square:Square = bitboard.trailing_zeros() as Square;
            let col = origin_square.get_col();
            let target_square:Square = origin_square + active_player.get_pawn_direction();
            let promotes = target_square.get_row() == promotion_row;
            bitboard &= !(1<<origin_square);

            //Add regular forward move
            if ! board.is_piece_at(target_square) {
                if promotes {
                    for promotion_piece in active_player.get_promotion_pieces().iter() {
                        moves.push(ChessMove::new(*promotion_piece, origin_square, target_square, MoveData::Promotion))
                    }
                } else {
                    moves.push(ChessMove::new(pawn, origin_square, target_square, MoveData::Normal));

                    //If regular move is valid, double move may be valid
                    if origin_square.get_row() == starting_row {
                        let target_square: Square = if pawn_row_ascending {origin_square + 16} else {origin_square - 16};
                        if !board.is_piece_at(target_square) {
                            moves.push(ChessMove::new(pawn, origin_square, target_square, MoveData::EnableEnPassant));
                        }
                    }
                }
            }

            fn diagonal_capture(board: &Board, moves: &mut Vec<ChessMove>, pawn: Piece, origin_square: Square,diagonal_target_square: Square, active_player: Color, promotes: bool){
                // Regular capture
                if board.is_piece_at(diagonal_target_square) {
                    if let Some(_) = board.get_colored_piece_at(diagonal_target_square, active_player.toggle_color()){
                        if promotes {
                            for promotion_piece in active_player.get_promotion_pieces().iter() {
                                moves.push(ChessMove::new(*promotion_piece, origin_square, diagonal_target_square, MoveData::CapturePromotion));
                            }
                        } else {
                            moves.push(ChessMove::new(pawn, origin_square, diagonal_target_square, MoveData::Capture));
                        }
                    }
                // en passant
                } else if board.en_passant_square == Some(diagonal_target_square) {
                    moves.push(ChessMove::new(pawn, origin_square, diagonal_target_square, MoveData::EnPassant));
                }
            }
            //Diagonal left Captures
            if col > 0 {
                let target_square:Square = target_square - 1;
                diagonal_capture(&board, moves, pawn, origin_square, target_square, active_player, promotes);
            }

            //Diagonal right Captures
            if col < 7 {
                let target_square:Square = target_square + 1;
                diagonal_capture(&board, moves, pawn, origin_square, target_square, active_player, promotes);
            }
        }
    }
    fn add_rook_moves(board: &Board, moves: &mut Vec<ChessMove>) {
        let active_player = board.active_player;
        let rook = active_player.get_rook();
        let mut bitboard = board.get_bitboard(rook);
        while bitboard != 0 {
            let origin_square:Square = bitboard.trailing_zeros() as Square;
            bitboard &= !(1<<origin_square);
            for i in 0..4 {
                let ascending = i/2 == 0;
                let result = if i%2 == 0 {
                    board.clear_n_capture_down_file(origin_square, ascending, active_player.toggle_color())
                } else {
                    board.clear_n_capture_down_rank(origin_square, ascending, active_player.toggle_color())
                };
                let mut movable_squares = result.0;
                let capture_square = result.1;
                while movable_squares != 0 {
                    let target_square = movable_squares.trailing_zeros() as Square;
                    movable_squares &= !(1<< target_square);
                    moves.push(ChessMove::new(rook, origin_square, target_square, MoveData::Normal));
                }
                if let Some(target_square) = capture_square {
                    moves.push(ChessMove::new(rook, origin_square, target_square, MoveData::Capture));
                }
            }
        }
    }
    fn add_knight_moves(board: &Board, moves: &mut Vec<ChessMove>) {

        let active_player = board.active_player;
        let opponent = active_player.toggle_color();
        let knight = active_player.get_knight();
        let mut bitboard = board.get_bitboard(knight);
        fn add_if_valid(board: &Board, moves: &mut Vec<ChessMove>, knight: Piece, origin_square: Square, target_square: Square, opponent: Color) {
            if board.is_piece_at(target_square) {
                if Some(board.get_colored_piece_at(target_square, opponent)).is_some() {
                    moves.push(ChessMove::new(knight, origin_square, target_square, MoveData::Capture));
                }
            } else {
                moves.push(ChessMove::new(knight, origin_square, target_square, MoveData::Normal));
            }
        }
        while bitboard != 0 {
            let origin_square:Square = bitboard.trailing_zeros() as Square;
            bitboard &= !(1<<origin_square);
            // Iterate through possible knight move offsets
            /*for row_offset in (-2i8)..=2 {
                if row_offset == 0 {continue;}
                for col_offset in (-2i8)..=2 {
                    if col_offset == 0 {continue;}
                    if row_offset.abs() == col_offset.abs() {continue;}
                    if let Some(target_square) = Square::valid_new(
                        (origin_square.get_row() as i8 + row_offset) as u8,
                        (origin_square.get_col() as i8 + col_offset) as u8
                    ) {
                        if board.is_piece_at(target_square) {
                            if let Some(target_piece) = board.get_colored_piece_at(target_square, opponent) {
                                moves.push(ChessMove::new(knight, origin_square, target_square, MoveData::Capture));
                            }
                        } else {
                            moves.push(ChessMove::new(knight, origin_square, target_square, MoveData::Normal));
                        }
                    }
                }
            }*/
            let row = origin_square.get_row();
            let col = origin_square.get_col();
            if let Some(target_square) = Square::valid_new(row - 2, col - 1) { add_if_valid(board, moves, knight, origin_square, target_square, opponent)};
            if let Some(target_square) = Square::valid_new(row - 2, col + 1) { add_if_valid(board, moves, knight, origin_square, target_square, opponent)};
            if let Some(target_square) = Square::valid_new(row - 1, col - 2) { add_if_valid(board, moves, knight, origin_square, target_square, opponent)};
            if let Some(target_square) = Square::valid_new(row - 1, col + 2) { add_if_valid(board, moves, knight, origin_square, target_square, opponent)};
            if let Some(target_square) = Square::valid_new(row + 1, col - 2) { add_if_valid(board, moves, knight, origin_square, target_square, opponent)};
            if let Some(target_square) = Square::valid_new(row + 1, col + 2) { add_if_valid(board, moves, knight, origin_square, target_square, opponent)};
            if let Some(target_square) = Square::valid_new(row + 2, col - 1) { add_if_valid(board, moves, knight, origin_square, target_square, opponent)};
            if let Some(target_square) = Square::valid_new(row + 2, col + 1) { add_if_valid(board, moves, knight, origin_square, target_square, opponent)};
        }
    }
    fn add_bishop_moves(board: &Board, moves: &mut Vec<ChessMove>) {
        let active_player = board.active_player;
        let bishop = active_player.get_bishop();
        let mut bitboard = board.get_bitboard(bishop);

        while bitboard != 0 {
            let origin_square:Square = bitboard.trailing_zeros() as Square;
            bitboard &= !(1<<origin_square);

            for i in 0..4 {
                let ascending_row = i/2 == 0;
                let ascending_col = i%2 == 0;
                let result = board.clear_n_capture_down_diagonal(origin_square, ascending_row, ascending_col, active_player.toggle_color());

                let mut movable_squares = result.0;
                let capture_square = result.1;

                while movable_squares != 0 {
                    let target_square = movable_squares.trailing_zeros() as Square;
                    movable_squares &= !(1<<target_square);
                    moves.push(ChessMove::new(bishop, origin_square, target_square, MoveData::Normal));
                }

                if let Some(target_square) = capture_square {
                    moves.push(ChessMove::new(bishop, origin_square, target_square, MoveData::Capture));
                }
            }
        }
    }
    fn add_queen_moves(board: &Board, moves: &mut Vec<ChessMove>) {
        let active_player = board.active_player;
        let queen = active_player.get_queen();
        let mut bitboard = board.get_bitboard(queen);

        while bitboard != 0 {
            let origin_square:Square = bitboard.trailing_zeros() as Square;
            bitboard &= !(1<<origin_square);

            for i in 0..4 {
                let ascending = i/2 == 0;
                let ascending_col = i%2 == 0;

                //Handle straight moves
                {
                    let result = if i % 2 == 0 {
                        board.clear_n_capture_down_rank(origin_square, ascending, active_player.toggle_color())
                    } else {
                        board.clear_n_capture_down_file(origin_square, ascending, active_player.toggle_color())
                    };
                    let mut movable_squares = result.0;
                    let capture_square = result.1;

                    while movable_squares != 0 {
                        let target_square = movable_squares.trailing_zeros() as Square;
                        movable_squares &= !(1 << target_square);
                        moves.push(ChessMove::new(queen, origin_square, target_square, MoveData::Normal));
                    }
                    if let Some(target_square) = capture_square {
                        moves.push(ChessMove::new(queen, origin_square, target_square, MoveData::Capture));
                    }
                }

                //Handle diagonal moves
                {
                    let result = board.clear_n_capture_down_diagonal(origin_square, ascending, ascending_col, active_player.toggle_color());
                    let mut movable_squares = result.0;
                    let capture_square = result.1;

                    while movable_squares != 0 {
                        let target_square = movable_squares.trailing_zeros() as Square;
                        movable_squares &= !(1 << target_square);
                        moves.push(ChessMove::new(queen, origin_square, target_square, MoveData::Normal));
                    }
                    if let Some(target_square) = capture_square {
                        moves.push(ChessMove::new(queen, origin_square, target_square, MoveData::Capture));
                    }
                }

            }
        }


    }
    fn add_king_moves(board: &Board, moves: &mut Vec<ChessMove>) {
        let active_player = board.active_player;
        let opponent = active_player.toggle_color();
        let king = active_player.get_king();
        let mut bitboard = board.get_bitboard(king);

        while bitboard != 0 {
            let origin_square:Square = bitboard.trailing_zeros() as Square;
            bitboard &= !(1<<origin_square);

            // Calculate regular king moves
            for col_offset in -1..1{
                for row_offset in -1..1{
                    if col_offset == 0 && row_offset == 0 {continue;}

                    if let Some(target_square) = Square::valid_new(origin_square.get_row() + row_offset as u8, origin_square.get_col() + col_offset as u8){
                        if board.is_piece_at(target_square) {
                            if board.get_colored_piece_at(target_square, opponent).is_some(){
                                moves.push(ChessMove::new(king, origin_square, target_square, MoveData::Capture));
                            }
                        } else {
                            moves.push(ChessMove::new(king, origin_square, target_square, MoveData::Normal));
                        }
                    }
                }
            }

            // Calculate castle
            match active_player {
                /*
                 * 0, 7, 56, and 63 are the starting locations of the rooks
                 * If the flag is enabled, the rooks will be there and the king in its original position
                 * To go up and down a row you need to subtract or add 8, to go left or right you add
                 * 1, so +2/-2 describes the kings movement in a Castle to a T.
                 */
                Color::White => {
                    if board.castling_rights.white_king_side {
                        if board.sees_down_rank(origin_square, true) & (1<<63) == 1<<63{
                            moves.push(ChessMove::new(king, origin_square, origin_square + 2, MoveData::Castling));
                        }
                    }
                    if board.castling_rights.white_queen_side {
                        if board.sees_down_rank(origin_square, true) & (1<<56) == 1<<56{
                            moves.push(ChessMove::new(king, origin_square, origin_square - 2, MoveData::Castling));
                        }
                    }
                },
                Color::Black => {
                    if board.castling_rights.black_king_side {
                        if board.sees_down_rank(origin_square, true) & (1<<7) == 1<<7{
                            moves.push(ChessMove::new(king, origin_square, origin_square + 2, MoveData::Castling));
                        }
                    }
                    if board.castling_rights.black_queen_side {
                        if board.sees_down_rank(origin_square, true) & (1<<0) == 1<<0{
                            moves.push(ChessMove::new(king, origin_square, origin_square - 2, MoveData::Castling));
                        }
                    }
                }
            }
        }

    }

    pub fn debug_string(&self) -> String {
        format!("{} {}", self.to_long_algebraic(), self.meta_data.to_string())
    }
}

impl Display for ChessMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.debug_string().fmt(f)
    }
}

pub fn special_move_builder(move_type: SpecialMoveType, result: MoveResult, captures: bool) -> MoveData {
    match move_type {
        SpecialMoveType::None => build_none_or_promotion(MoveData::Normal, captures, MoveData::Capture, result),
        SpecialMoveType::Promotion => build_none_or_promotion(MoveData::Promotion, captures, MoveData::CapturePromotion, result),
        SpecialMoveType::Castling => build_move_type(MoveData::Castling, result),
        SpecialMoveType::EnPassant => build_move_type(MoveData::EnPassant, result), // Captures handled intrinsically
        SpecialMoveType::EnableEnPassant => build_move_type(MoveData::EnableEnPassant, result),
    }
}
/// Builds `Normal`- or promotion-related moves
fn build_none_or_promotion(base: MoveData, captures: bool, capture_base: MoveData, result: MoveResult) -> MoveData {
    match result {
        MoveResult::None => if captures { capture_base } else { base },
        MoveResult::Check => if captures { capture_base.add_check() } else { base.add_check() },
        MoveResult::Checkmate => if captures { capture_base.add_checkmate() } else { base.add_checkmate() },
        MoveResult::Stalemate => if captures { capture_base.add_stalemate() } else { base.add_stalemate() },
    }
}

/// Builds moves for types without implicit captures (e.g. `Castling`, `EnPassant`, `EnableEnPassant`)
fn build_move_type(base: MoveData, result: MoveResult) -> MoveData {
    match result {
        MoveResult::None => base,
        MoveResult::Check => base.add_check(),
        MoveResult::Checkmate => base.add_checkmate(),
        MoveResult::Stalemate => base.add_stalemate(),
    }
}

/// Helper methods for modifying SpecialMoves
impl MoveData {
    pub fn get_move_type(&self) -> MoveType {
        match self{
            MoveData::Normal | MoveData::NormalCheck | MoveData::NormalCheckmate | MoveData::NormalStalemate |
            MoveData::Capture | MoveData::CaptureCheck | MoveData::CaptureCheckmate | MoveData::CaptureStalemate |
            MoveData::EnableEnPassant | MoveData::EnableEnPassantCheck | MoveData::EnableEnPassantCheckmate | MoveData::EnableEnPassantStalemate
                => MoveType::Regular,
            MoveData::EnPassant | MoveData::EnPassantCheck | MoveData::EnPassantCheckmate | MoveData::EnPassantStalemate
                => MoveType::EnPassant,
            MoveData::Promotion | MoveData::PromotionCheck | MoveData::PromotionCheckmate | MoveData::PromotionStalemate |
            MoveData::CapturePromotion | MoveData::CapturePromotionCheck | MoveData::CapturePromotionCheckmate | MoveData::CapturePromotionStalemate
                => MoveType::Promotion,
            MoveData::Castling | MoveData::CastlingCheck | MoveData::CastlingCheckmate | MoveData::CastlingStalemate
                => MoveType::Castling,
            MoveData::GeneratedOnly => !unreachable!()
        }
    }
    pub fn is_castle(&self) -> bool {
        match self {
            MoveData::Castling | MoveData::CastlingCheck | MoveData::CastlingCheckmate |
            MoveData::CastlingStalemate => true,
            _ => false,
        }
    }
    pub fn is_promotion(&self) -> bool {
        match self {
            MoveData::Promotion | MoveData::PromotionCheck | MoveData::PromotionCheckmate |
            MoveData::PromotionStalemate | MoveData::CapturePromotion |
            MoveData::CapturePromotionCheck | MoveData::CapturePromotionCheckmate |
            MoveData::CapturePromotionStalemate=> true,
            _ => false,
        }
    }
    pub fn is_check(&self) -> bool {
        match self {
            MoveData::NormalCheck | MoveData::CastlingCheck | MoveData::PromotionCheck |
            MoveData::CapturePromotionCheck | MoveData::EnPassantCheck |
            MoveData::EnableEnPassantCheck => true,
            _ => false,
        }
    }
    pub fn is_checkmate(&self) -> bool {
        match self {
            MoveData::NormalCheckmate | MoveData::CastlingCheckmate | MoveData::PromotionCheckmate |
            MoveData::CapturePromotionCheckmate | MoveData::EnPassantCheckmate |
            MoveData::EnableEnPassantCheckmate => true,
            _ => false,
        }
    }
    pub fn is_stalemate(&self) -> bool {
        match self {
            MoveData::NormalStalemate | MoveData::CastlingStalemate | MoveData::PromotionStalemate |
            MoveData::CapturePromotionStalemate | MoveData::EnPassantStalemate |
            MoveData::EnableEnPassantStalemate => true,
            _ => false,
        }
    }
    pub fn is_capture(&self) -> bool {
        match self {
            MoveData::Capture | MoveData::CaptureCheck | MoveData::CaptureCheckmate | MoveData::CaptureStalemate |
            MoveData::CapturePromotion | MoveData::CapturePromotionCheck | MoveData::CapturePromotionCheckmate | MoveData::CapturePromotionStalemate |
            MoveData::EnPassant | MoveData::EnPassantCheck | MoveData::EnPassantCheckmate | MoveData::EnPassantStalemate
                => true,
            _ => false,
        }
    }
    pub fn is_en_passant(&self) -> bool {
        match self {
            MoveData::EnPassant | MoveData::EnPassantCheck | MoveData::EnPassantCheckmate |
            MoveData::EnPassantStalemate => true,
            _ => false,
        }
    }
    pub fn is_enable_en_passant(&self) -> bool {
        match self {
            MoveData::EnableEnPassant | MoveData::EnableEnPassantCheck |
            MoveData::EnableEnPassantCheckmate | MoveData::EnableEnPassantStalemate => true,
            _ => false,
        }
    }
    pub fn reset_halfmove(&self) -> bool {
        match self{
            MoveData::Capture | MoveData::CaptureCheck | MoveData::CaptureCheckmate | MoveData::CaptureStalemate |
            MoveData::EnPassant | MoveData::EnPassantCheck | MoveData::EnPassantCheckmate | MoveData::EnPassantStalemate |
            MoveData::Promotion | MoveData::PromotionCheck | MoveData::PromotionCheckmate | MoveData::PromotionStalemate |
            MoveData::CapturePromotion | MoveData::CapturePromotionCheck | MoveData::CapturePromotionCheckmate | MoveData::CapturePromotionStalemate
                => true,
            _ => false,
        }
    }
    pub fn move_result(&self) -> MoveResult {
        match self {
            MoveData::Normal | MoveData::Castling | MoveData::Promotion |
            MoveData::EnPassant | MoveData::EnableEnPassant |
            MoveData::Capture | MoveData::CapturePromotion =>
                MoveResult::None,
            MoveData::NormalCheck | MoveData::CastlingCheck | MoveData::PromotionCheck |
            MoveData::EnPassantCheck | MoveData::EnableEnPassantCheck |
            MoveData::CaptureCheck | MoveData::CapturePromotionCheck =>
                MoveResult::Check,
            MoveData::NormalCheckmate | MoveData::CastlingCheckmate | MoveData::PromotionCheckmate |
            MoveData::EnPassantCheckmate | MoveData::EnableEnPassantCheckmate |
            MoveData::CaptureCheckmate | MoveData::CapturePromotionCheckmate=>
                MoveResult::Checkmate,
            MoveData::NormalStalemate | MoveData::CastlingStalemate | MoveData::PromotionStalemate |
            MoveData::EnPassantStalemate | MoveData::EnableEnPassantStalemate |
            MoveData::CaptureStalemate | MoveData::CapturePromotionStalemate =>
                MoveResult::Stalemate,
            MoveData::GeneratedOnly | _ => unimplemented!("GeneratedOnly variant not handled for {:?}", self),
        }
    }
    fn add_check(self) -> MoveData {
        match self {
            MoveData::Normal => MoveData::NormalCheck,
            MoveData::Castling => MoveData::CastlingCheck,
            MoveData::Promotion => MoveData::PromotionCheck,
            MoveData::CapturePromotion => MoveData::CapturePromotionCheck,
            MoveData::EnPassant => MoveData::EnPassantCheck,
            MoveData::EnableEnPassant => MoveData::EnableEnPassantCheck,
            _ => unimplemented!("Check variant not handled for {:?}", self),
        }
    }
    fn add_checkmate(self) -> MoveData {
        match self {
            MoveData::Normal => MoveData::NormalCheckmate,
            MoveData::Castling => MoveData::CastlingCheckmate,
            MoveData::Promotion => MoveData::PromotionCheckmate,
            MoveData::CapturePromotion => MoveData::CapturePromotionCheckmate,
            MoveData::EnPassant => MoveData::EnPassantCheckmate,
            MoveData::EnableEnPassant => MoveData::EnableEnPassantCheckmate,
            _ => unimplemented!("Checkmate variant not handled for {:?}", self),
        }
    }
    fn add_stalemate(self) -> MoveData {
        match self {
            MoveData::Normal => MoveData::NormalStalemate,
            MoveData::Castling => MoveData::CastlingStalemate,
            MoveData::Promotion => MoveData::PromotionStalemate,
            MoveData::CapturePromotion => MoveData::CapturePromotionStalemate,
            MoveData::EnPassant => MoveData::EnPassantStalemate,
            MoveData::EnableEnPassant => MoveData::EnableEnPassantStalemate,
            _ => unimplemented!("Stalemate variant not handled for {:?}", self),
        }
    }

    fn to_string(&self) -> String {
        match self {
            MoveData::Normal => String::from("Normal"),
            MoveData::NormalCheck => String::from("Check"),
            MoveData::NormalCheckmate => String::from("Checkmate"),
            MoveData::NormalStalemate => String::from("Stalemate"),
            MoveData::Castling => String::from("Castling"),
            MoveData::CastlingCheck => String::from("CastlingCheck"),
            MoveData::CastlingCheckmate => String::from("CastlingCheckmate"),
            MoveData::CastlingStalemate => String::from("CastlingStalemate"),
            MoveData::Promotion => String::from("Promotion"),
            MoveData::PromotionCheck => String::from("PromotionCheck"),
            MoveData::PromotionCheckmate => String::from("PromotionCheckmate"),
            MoveData::PromotionStalemate => String::from("PromotionStalemate"),
            MoveData::EnPassant => String::from("EnPassant"),
            MoveData::EnPassantCheck => String::from("EnPassantCheck"),
            MoveData::EnPassantCheckmate => String::from("EnPassantCheckmate"),
            MoveData::EnPassantStalemate => String::from("EnPassantStalemate"),
            MoveData::EnableEnPassant => String::from("EnableEnPassant"),
            MoveData::EnableEnPassantCheck => String::from("EnableEnPassantCheck"),
            MoveData::EnableEnPassantCheckmate => String::from("EnableEnPassantCheckmate"),
            MoveData::EnableEnPassantStalemate => String::from("EnableEnPassantStalemate"),
            MoveData::Capture => String::from("Capture"),
            MoveData::CaptureCheck => String::from("CaptureCheck"),
            MoveData::CaptureCheckmate => String::from("CaptureCheckmate"),
            MoveData::CaptureStalemate => String::from("CaptureStalemate"),
            MoveData::CapturePromotion => String::from("CapturePromotion"),
            MoveData::CapturePromotionCheck => String::from("CapturePromotionCheck"),
            MoveData::CapturePromotionCheckmate => String::from("CapturePromotionCheckmate"),
            MoveData::CapturePromotionStalemate => String::from("CapturePromotionStalemate"),
            //Moves may be generated by the engine ahead of time, so they may need to be properly classified.
            MoveData::GeneratedOnly => String::from("GeneratedOnly"),
        }
    }
}