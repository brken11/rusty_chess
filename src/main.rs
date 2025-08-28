mod ai;
mod board;
mod chess_bot;
mod chess_moves;
mod clock;
mod common;
mod config;
mod game;
mod handler;
mod log;
mod move_parser;
mod player_agent;
mod rules;
mod time;
mod ui;

use std::thread;

use crate::board::pieces::Piece;
use crate::board::Board;
use crate::chess_moves::ChessMove;
use crate::common::{ThreadIdentifier};
use crate::common::common_lib;
use crate::config::{parse_config, Config, ConfigResult};
use crate::log::{LogLevel, LogMessage, LogOutput};
use crate::ui::{UIManager,UIType};

fn main() {
    let mut b = Board::std_new();
    let mut e = Board::empty_new();
    println!("{}", b.to_string());
    println!("{}", if let Some(square) = b.king_square() {square} else {255});
    println!("{}", e.to_string());
    println!("{}", if let Some(square) = e.king_square() {square} else {255});
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

    let (config, config_result) = parse_config();
    let ui_type: UIType = config.ui_type;
    let log_output: LogOutput = config.log_output;

    // Initialize logger
    let main_id = init_main();
    let (log_handle, log_channel) = common_lib::init_logger(main_id, log_output);
    common_lib::set_log_thread_log_level(main_id, LogLevel::Debug);
    // Report config results
    match config_result {
        ConfigResult::Ok => {
            let _ = log_channel.send(LogMessage::Message(main_id, LogLevel::Debug, "Config loaded and parsed!".to_string()));
        }
        ConfigResult::InvalidUtf8(message) => {
            let _ = log_channel.send(LogMessage::Message(main_id, LogLevel::Warning, message));
        }
        ConfigResult::NoConfigFile(message) => {
            let _ = log_channel.send(LogMessage::Message(main_id, LogLevel::Warning, message));
        }
        ConfigResult::ParsingError(error_messages) => {
            for message in error_messages {
                let r = log_channel.send(LogMessage::Message(main_id, LogLevel::Error, message));
                if let Err(_) = r {
                    println!("Log panicked.");
                    break;
                }
            }
        }
    }

    // Initialize ui
    let mut ui_thread = UIManager::new(Some(log_channel.clone()));
    ui_thread.set_ui_type(ui_type);
    let ui_handle = ui_thread.start();

    let test = log_channel.send(LogMessage::Message(
        main_id,
        LogLevel::Info,
        "Test message".to_string(),
    ));

    if test.is_err() {
        return;
    }

    thread::sleep(std::time::Duration::from_millis(1000));

    println!("Test shutdown from main thread");
    let test = log_channel.send(LogMessage::Instruction(
        main_id,
        log::LogInstruction::Shutdown,
    ));
    if test.is_err() {
        println!("LogThread panicked");
    }

    let ui_thread = ui_handle.join();
    if ui_thread.is_err() {
        println!("UI panicked");
    }

    thread::sleep(std::time::Duration::from_millis(1000));
    
    println!("End of main");
}

fn init_main() -> ThreadIdentifier {
    let hash: u128 = ThreadIdentifier::generate_id();
    ThreadIdentifier::Main(hash)
}
