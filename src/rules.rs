use std::time::{Duration, Instant};
use crate::board::Color;

/// Describes the status of the Chess game
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

pub struct TimeControls {
    pub initial_time_white: Duration,
    pub initial_time_black: Duration,
    pub time_per_move_white: Duration,
    pub time_per_move_black: Duration,
}

/// A trait representing the basic functionalities of a chess clock.
/// A chess clock is used to manage and track the time for two players in a game.
///
/// # Key Features
/// - Control the timing for two players via start, stop, and switching mechanics.
/// - Track the remaining time of both players individually.
/// - Manage time allocations and resets.
pub trait Timer {

    /// Method for returning a new instance
    fn new_from_time_controls(time_controls: TimeControls) -> Option<Self>
        where Self: Sized;

    /// Starts the clock for the current active player.
    fn start(&mut self);
    /// Stops the clock without switching players.
    fn stop(&mut self);
    /// Switches the active player and starts their clock.
    fn switch_clock(&mut self);
    /// Resets both players' clocks to their initial time.
    fn reset(&mut self);
    /// Returns true if the clock is currently running.
    fn is_running(&self) -> bool;
    /// Returns the player whose clock is currently active.
    fn active_player(&self) -> Color;

    /// Returns the total time originally allocated per player.
    fn get_total_time(&self) -> Duration;
    /// Returns the shared remaining time (used for synchronizing or bulk resets).
    fn get_time_left(&self) -> Duration;
    /// Returns the remaining time for the active player.
    fn get_active_time_left(&self) -> Duration;
    /// Returns the remaining time for a specific player.
    fn get_player_time_left(&self, color: Color) -> Duration;

    /// Sets the shared total time for both players (e.g., for resets or mirroring).
    fn set_total_time(&mut self, time: Duration);
    /// Sets the remaining time for a specific player.
    fn set_player_time_left(&mut self, color: Color, time: Duration);
}