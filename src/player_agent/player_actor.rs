use crate::board::{Board, Color, Square};
use crate::chess_moves::{ChessMove, MoveError};
use crate::move_parser::{chess_notation_parser, ParseError, ProtoMove};

pub trait Player {
    // Game Manager requests
    fn on_illegal_move(&mut self, error: ParseError);

    // GameManager commands
    fn sync();
    fn sync_clock();
    fn sync_board();
    fn sync_turn();

    // User requests
    fn submit_move(chess_move: ChessMove);
    fn request_sync();

    // Internal methods
    fn get_board(&self) -> Board;
    fn get_active_player(&self) -> Color;
    fn parse_short_algebraic(&self, chess_move: String) -> Result<ChessMove, ParseError> {
        let active_player = self.get_active_player();
        let proto_move = chess_notation_parser::from_simplified_algebraic_notation(
            chess_move.as_str(),
            active_player,
        )?;
        let mut board = self.get_board();

        ChessMove::new_from_proto(&mut board, proto_move)
    }
    fn parse_squares(&self, origin: Square, target: Square) -> Result<ChessMove, MoveError> {
        let mut board = self.get_board();

        ChessMove::new_from_squares(&mut board, origin, target, false)
    }
}
