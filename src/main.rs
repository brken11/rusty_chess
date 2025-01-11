mod board;
mod chess_moves;
mod rules;

use board::Board;

fn main() {
    let mut b = Board::std_new();
    println!("{}", b.to_string());
}
