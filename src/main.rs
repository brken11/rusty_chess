mod board;
mod chess_moves;
mod rules;

use board::Board;
use crate::board::pieces::Color;

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
    println!("{}", Color::White.get_pawn_direction()*2);
}
