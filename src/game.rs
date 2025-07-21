use std::cmp::PartialEq;
use crate::rules;
use crate::board::Board;
use crate::chess_moves::{ChessMove, MoveError};
use crate::rules::{GameState, TimeControls, Timer};
use crate::clock::ChessClock;
use crate::common::{ThreadIdHash, ThreadIdHashExt, ThreadIdentifier};
// use crate::ai::ChessAI;

use std::thread;
use std::sync::mpsc::{Receiver,Sender};
use std::time::Instant;
use crate::handler::PlayerHandler;
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

#[derive(Debug, Clone, Copy)]
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
}

pub type FullMoveNumber = rules::FullMoveNumber;

pub struct GameThread{
    game : Game,
    thread_identifier: ThreadIdentifier,
    player_1_handler: Option<PlayerHandler>,
    player_2_handler: Option<PlayerHandler>,
    ui_in : Receiver<GameController>,
    move_history: Vec<ChessMove>,
}

pub enum GameMessage {
    RequestSync,
    RequestSyncClock,
    RequestSyncMoveHistory,
    MakeMove(ChessMove, FullMoveNumber),
    SetPremove(ChessMove, FullMoveNumber),
}
pub enum GameResponse {
    Sync(Game),
    SyncClock(ChessClock),
    SyncMoveHistory(Vec<ChessMove>),
    IllegalMove(MoveError),
}

pub enum GameController {
    StartGame,
    StopGame,
    AbortThread,
}

impl Game{
    pub fn new(white: PlayerType, black: PlayerType) -> Game {
        Game{
            board: Board::std_new(),
            white,
            black,
            game_state: GameState::Running,
            clock: None,
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
        })
    }
    fn start_game(&mut self) {
        if self.game_state == GameState::Start {
            if let Some(clock) = &mut self.clock {
                clock.start();
            }
            self.game_state = GameState::Running;
        }
    }
}

impl GameThread {
    ///
    ///
    ///
    ///
    ///
    ///
    pub fn new(game: Game) -> (GameThread, Sender<GameController>) {
        let thread_identifier = ThreadIdentifier::Game(ThreadIdHash::new());
        // let (player_1_in_sender, player_1_in_receiver) = std::sync::mpsc::channel();
        // let (player_2_in_sender, player_2_in_receiver) = std::sync::mpsc::channel();
        // let (player_1_out_sender, player_1_out_receiver) = std::sync::mpsc::channel();
        // let (player_2_out_sender, player_2_out_receiver) = std::sync::mpsc::channel();
        let (ui_out, ui_in) = std::sync::mpsc::channel();

        // let player_1_handler = PlayerHandler::new(game.white, player_1_in_sender, player_1_out_receiver);
        // let player_2_handler = PlayerHandler::new(game.black, player_2_in_sender, player_2_out_receiver);

        (
            GameThread {
                thread_identifier,
                game,
                player_1_handler: None,
                player_2_handler: None,
                ui_in,
                move_history: Vec::new(),
            }
            , ui_out
        )
    }
    ///
    ///
    ///
    ///
    ///
    ///
    pub fn start(mut self) -> Self {
        self.game.start_game();

        self.run();

        self
    }
    fn run(&mut self) {
        loop {

        }
    }
}
