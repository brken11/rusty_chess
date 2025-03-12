mod board;
mod chess_moves;
mod rules;

use crate::board::pieces::Piece;
use board::Board;
use crate::board::pieces::Color;
use crate::chess_moves::ChessMove;

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
    e.set_bitboard(Piece::WhiteQueen, 0x00000000004000000);
    println!("{}", e.to_string());
    let possible_moves: Vec<ChessMove> = ChessMove::get_valid_moves(&mut b);
    for m in possible_moves {
        println!("{}", m.debug_string());
    }
}
