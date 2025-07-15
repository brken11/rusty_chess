use crate::board::Board;
use crate::chess_moves::ChessMove;
use crate::rules::{GameState, TimeControls, Timer};
use crate::clock::ChessClock;
// use crate::ai::ChessAI;

use std::thread;
use std::sync::Mutex;
use std::time::Instant;
// pub trait AIBehavior {
//     fn make_move(&self, board: &Board) -> Result<ChessMove, AIError>;
// }

/// Represents the type of player in a chess game.
///
/// This enumeration defines various types of players that can participate in the game.
/// Each variant encapsulates specific characteristics of the player,
/// whether they are human, AI, or bot, as well as their mode of connection.
///
/// # Variants
///
/// - `LocalHuman`
///     A human player playing on the local system.
/// - `RemoteHuman`
///     A human player playing through a network connection.
/// - `LocalAI(ChessAI)`
///     An AI player running locally on the system. The `ChessAI` parameter
///     provides additional details or configuration about the AI.
/// - `RemoteAI(ChessAI)`
///     An AI player connected through a network. The `ChessAI` parameter
///     provides additional details or configuration about the AI.
/// - `RemoteBot(ChessBot)`
///     A bot player or API-based player, capable of automated gameplay.
///     The `ChessBot` parameter provides additional details or configuration
///     about the bot.
pub enum PlayerType{
    /// Human player playing locally
    LocalHuman,
    /// Human player playing through a network connection
    RemoteHuman,
    // /// AI player playing locally
    // LocalAI(ChessAI),
    // /// AI player playing through a network connection
    // RemoteAI(ChessAI),
    // /// Bot player/API
    // RemoteBot(ChessBot),
}

pub struct GameMetadata {
    game_id: Option<String>,
    start_time: Instant,
    white_rating: Option<u32>,
    black_rating: Option<u32>,
}

pub struct Game{
    board: Board,
    white: PlayerType,
    black: PlayerType,
    game_state: GameState,
    clock: Option<ChessClock>,
    move_history: Vec<ChessMove>,
}

impl Game{
    pub fn new(white: PlayerType, black: PlayerType) -> Game {
        Game{
            board: Board::std_new(),
            white,
            black,
            game_state: GameState::Running,
            clock: None,
            move_history: Vec::new(),
        }
    }
    pub fn new_with_time_controls(white: PlayerType, black: PlayerType, time_controls: TimeControls) -> Option<Game> {
        let clock = match ChessClock::new_from_time_controls(time_controls) {
            Some(clock) => Some(clock),
            None => return None,
        };
        Some(Game{
            board: Board::std_new(),
            white,
            black,
            game_state: GameState::Running,
            clock,
            move_history: Vec::new(),
        })
    }
    pub fn start(&mut self) {
        if let Some(clock) = &mut self.clock {
            clock.start();
        }
    }
}