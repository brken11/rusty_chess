mod pieces;
use pieces::Color;
use pieces::Piece;


pub struct Board {
    pub can_castle_white_king_side: bool,
    pub can_castle_white_queen_side: bool,
    pub can_castle_black_king_side: bool,
    pub can_castle_black_queen_side: bool,
    pub en_passant_square: Option<u8>,
    pub half_move_clock: u8,
    pub full_move_number: u8,
    pub active_player: Color,
    data: [u64; 12],
}

impl Board {
    pub fn std_new() -> Board {
        Board{
            can_castle_white_king_side : true,
            can_castle_white_queen_side : true,
            can_castle_black_king_side : true,
            can_castle_black_queen_side : true,
            en_passant_square : None,
            half_move_clock : 0,
            full_move_number : 1,
            active_player : Color::White,
            data : [
                // Whites         // Blacks
                0x00FF000000000000,0x000000000000FF00, // Pawns
                0x8100000000000000,0x0000000000000081, // Rooks
                0x4200000000000000,0x0000000000000042, // Knights
                0x2400000000000000,0x0000000000000024, // Bishops
                0x0800000000000000,0x0000000000000008, // Queens
                0x1000000000000000,0x0000000000000010, // Kings
            ],
        }
    }
    pub fn empty_new() -> Board {
        Board{
            can_castle_white_king_side : false,
            can_castle_white_queen_side : false,
            can_castle_black_king_side : false,
            can_castle_black_queen_side : false,
            en_passant_square : None,
            half_move_clock : 0,
            full_move_number : 1,
            active_player : Color::White,
            data : [0; 12],
        }
    }

    pub fn get_bitboard(&self, piece: Piece) -> u64 {
        self.data[piece as usize]
    }
    pub fn set_bitboard(&mut self, piece: Piece, bitboard: u64) {
        self.data[piece as usize] = bitboard;
    }

    pub fn get_piece_at(&self, square: u8) -> Option<Piece> {
        for piece in pieces::Piece::iter() {
            if self.data[piece as usize] & (1 << square) != 0 {
                return Some(piece);
            }
        }
        None
    }
    pub fn get_str_at(&self, square: u8) -> &str {
        if let Some(piece) = self.get_piece_at(square) {
            return piece.to_string().as_str();
        }
        "-"
    }

    pub fn to_string(&self) -> String {
        let mut rendered_board = String::new();
        rendered_board.push_str("   | a  b  c  d  e  f  g  h |\n");
        for rank in (1..=8).rev() {
            let r = (rank - 1) * 8; // Compute the starting index for this rank
            rendered_board.push_str(&format!(
                " {} | {}  {}  {}  {}  {}  {}  {}  {} |\n",
                rank,
                self.get_str_at(r),
                self.get_str_at(r + 1),
                self.get_str_at(r + 2),
                self.get_str_at(r + 3),
                self.get_str_at(r + 4),
                self.get_str_at(r + 5),
                self.get_str_at(r + 6),
                self.get_str_at(r + 7)
            ));
        }

        rendered_board
    }
}