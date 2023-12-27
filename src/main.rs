use std::time::Instant;

use crate::board::{Board, create_board, Placement};
use crate::pieces::{Piece, shape_from_template};

mod pieces;
mod board;

fn print_state(board: &Board) {
    for row in board.index_grid() {
        print!("|");
        for index in row {
            if index == -1 {
                print!(" ");
            } else {
                print!("{:x}", index);
            }
        }
        println!("|");
    }
}

fn place_pieces<'a>(top_level: bool, board: &mut Board<'a>, remaining: &'a [Vec<Piece>]) -> bool {
    for (i, transform) in (&remaining[0]).iter().enumerate() {
        if top_level {
            println!("{}%", (i * 100) / &remaining[0].len());
        }
        for column in 0..(1 + board.width - transform.width) {
            for row in 0..(1 + board.height - transform.height) {
                let placement = Placement {
                    column,
                    row,
                    piece: transform,
                };

                if board.try_add(placement) {
                    if remaining.len() == 1 || place_pieces(false, board, &remaining[1..]) {
                        return true;
                    } else {
                        board.remove_last();
                    }
                }
            }
        }
    }

    false
}

fn main() {
    let shapes: Vec<Vec<Piece>> = vec![
        // shape_from_template(1, vec!["*****"]),              //  1
        shape_from_template(2, vec!["*...", "****"]),       //  2
        shape_from_template(3, vec![".*..", "****"]),       //  3
        shape_from_template(4, vec!["**..", ".***"]),       //  4
        shape_from_template(5, vec!["*..", "*..", "***"]),  //  5
        shape_from_template(6, vec!["***", "**."]),         //  6
        // shape_from_template(7, vec!["*.*", "***"]),         //  7
        shape_from_template(8, vec!["*..", "***", "..*"]),  //  8
        // shape_from_template(9, vec!["*..", "***", ".*."]),  //  9
        shape_from_template(10, vec!["***", ".*.", ".*."]), // 10
        shape_from_template(11, vec!["**.", ".**", "..*"]), // 11
        // shape_from_template(12, vec![".*.", "***", ".*."]), // 12
    ].iter()
        .map(Piece::all_transforms)
        .collect();
    let mut board = create_board(8, 5);

    let start = Instant::now();
    if place_pieces(true, &mut board, shapes.as_slice()) {
        let elapsed = start.elapsed();
        println!("found solution in {}ms!", elapsed.as_millis());
        print_state(&board);
    } else {
        println!("no solution found :(");
    }
}

#[cfg(test)]
mod tests {
    use crate::{place_pieces, print_state};
    use crate::board::create_board;
    use crate::pieces::{Piece, shape_from_template};

    #[test]
    fn can_place_pieces() {
        let pieces: Vec<Vec<Piece>> = vec![
            shape_from_template(1, vec!["*.*", "***"]),
            shape_from_template(2, vec!["*.*", "***"]),
            shape_from_template(3, vec![".*.", "***", ".*."]),
        ].iter().map(Piece::all_transforms).collect();
        let mut board = create_board(5, 3);

        assert!(place_pieces(true, &mut board, pieces.as_slice()));
        print_state(&board);
        assert!(true)
    }
}