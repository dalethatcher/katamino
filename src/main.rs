use std::sync::Arc;
use std::time::Instant;

use crate::board::create_board;
use crate::pieces::{piece_from_name, PentominoName, Piece};

mod board;
mod pieces;

fn main() {
    let pieces = vec![
        piece_from_name(94, PentominoName::I),  //  1
        piece_from_name(208, PentominoName::L), //  2
        piece_from_name(130, PentominoName::Y), //  3
        piece_from_name(127, PentominoName::N), //  4
        piece_from_name(4, PentominoName::V),   //  5
        piece_from_name(217, PentominoName::P), //  6
        piece_from_name(11, PentominoName::U),  //  7
        piece_from_name(6, PentominoName::Z),   //  8
        piece_from_name(252, PentominoName::F), //  9
        piece_from_name(28, PentominoName::T),  // 10
        piece_from_name(10, PentominoName::W),  // 11
        piece_from_name(1, PentominoName::X),   // 12
    ];

    #[cfg(feature = "trace")]
    {
        for piece in pieces.iter() {
            println!("piece {}:", piece.id);
            println!("{}", piece.shape_string());
        }
    }

    let transforms: Arc<Vec<Vec<Piece>>> =
        Arc::new(pieces.iter().map(Piece::all_transforms).collect());
    let mut board = create_board(12, 5);

    let start = Instant::now();
    let solutions = board.find_solutions(&transforms);
    let elapsed = start.elapsed();

    if solutions.is_empty() {
        println!("no solution found :( in {} ms", elapsed.as_millis());
    } else {
        println!(
            "found {} solutions in {}ms!",
            solutions.len(),
            elapsed.as_millis()
        );
    }
}
