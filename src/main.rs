mod board;
mod chess_moves;
mod rules;
mod move_parser;
mod game;
mod ai;
mod clock;
mod chess_bot;
mod handler;
mod log;
mod common;

use std::thread;
use crate::board::pieces::Piece;
use board::Board;
use crate::chess_moves::ChessMove;
use crate::common::ThreadIdentifier;
use crate::log::{LogMessage, LogLevel};

fn main() {
    let mut b = Board::std_new();
    let mut e = Board::empty_new();
    println!("{}", b.to_string());
    println!("{}", if let Some(square)= b.king_square() {square} else {255});
    println!("{}", e.to_string());
    println!("{}", if let Some(square)= e.king_square() {square} else {255});
    // println!("Diagonal test false true");
    // b.sees_down_diagonal(8, false, true);
    // println!("Diagonal test false false");
    // b.sees_down_diagonal(8, false, false);
    // println!("Diagonal test true false");
    // b.sees_down_diagonal(8, true, false);
    // println!("Diagonal test true true");
    // b.sees_down_diagonal(8, true, true);
    // println!("{}", Color::White.get_pawn_direction()*2);
    e.set_bitboard(Piece::WhitePawn, 0x00FF000000000000);
    e.set_bitboard(Piece::WhiteRook, 0x0000008000000000);
    e.set_bitboard(Piece::BlackRook, 0x0000000200000000);
    e.set_bitboard(Piece::BlackPawn, 0x0000300000000000);
    e.set_bitboard(Piece::BlackBishop, 0x00000000000000080);
    e.set_bitboard(Piece::WhiteQueen, 0x00000000004000000);
    e.set_bitboard(Piece::WhiteKing, 0x0100000000000000);
    println!("{}", e.to_string());
    let possible_moves: Vec<ChessMove> = ChessMove::get_valid_moves(&mut b);
    for m in possible_moves {
        println!("{}", m.debug_string());
    }

    // if let Some(c)= move_parser::chess_notation_parser::normalize_piece_symbol("👸"){
    //     println!("Normalized text {}", c.to_string());
    // }

    let main_id = init_main();
    let (logger, log_channel, log_id) = log::LogThread::new(main_id);
    logger.start();

    let test = log_channel.send(LogMessage::Message(main_id, LogLevel::Info, "Test message".to_string()));

    if test.is_err() {
        return;
    }

    thread::sleep(std::time::Duration::from_millis(1000));

    println!("Test shutdown from main thread");
    let test = log_channel.send(LogMessage::Instruction(main_id, log::LogInstruction::Shutdown));

    if test.is_err() {
        println!("Thread panicked");
    }
    thread::sleep(std::time::Duration::from_millis(1000));

    println!("End of main");
}

pub fn init_main() -> ThreadIdentifier {
    let hash: u128 = std::time::Instant::now().elapsed().as_nanos();
    ThreadIdentifier::Main(hash)
}
