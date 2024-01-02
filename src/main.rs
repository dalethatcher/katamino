use std::sync::Arc;
use std::time::Instant;

use crate::board::create_board;
use crate::pieces::{shape_from_template, Piece};

mod board;
mod pieces;

fn main() {
    let pieces = vec![
        shape_from_template(94, vec!["*****"]),              //  1
        shape_from_template(208, vec!["*...", "****"]),      //  2
        shape_from_template(130, vec![".*..", "****"]),      //  3
        shape_from_template(127, vec!["**..", ".***"]),      //  4
        shape_from_template(4, vec!["*..", "*..", "***"]),   //  5
        shape_from_template(217, vec!["***", "**."]),        //  6
        shape_from_template(11, vec!["*.*", "***"]),         //  7
        shape_from_template(6, vec!["*..", "***", "..*"]),   //  8
        shape_from_template(252, vec!["*..", "***", ".*."]), //  9
        shape_from_template(28, vec!["***", ".*.", ".*."]),  // 10
                                                             // shape_from_template(10, vec!["**.", ".**", "..*"]),  // 11
                                                             // shape_from_template(1, vec![".*.", "***", ".*."]),   // 12
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
    let mut board = create_board(10, 5);

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
