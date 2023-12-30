use std::cell::RefCell;
use std::time::Instant;

use crate::board::{Board, create_board, Placement};
use crate::pieces::{Piece, shape_from_template};

mod pieces;
mod board;

fn print_state(board: &Board) {
    for row in board.piece_id_grid() {
        for piece_id in row {
            if piece_id == -1 {
                print!("\u{001b}[0m ");
            } else {
                print!("\u{001b}[48;5;{}m ", piece_id);
            }
        }
        println!("\u{001b}[0m");
    }
}

fn place_pieces<'a>(top_level: bool, board: &mut Board<'a>, remaining: &'a [Vec<Piece>]) -> bool {
    // reduces runtime to 1/4-1/8 of previous, but specific for this puzzle
    if board.contains_isolated_single() {
        return false;
    }

    let tracker = if !top_level {
        None
    } else {
        let number_of_possibilities = board.number_of_possibilities(&remaining[0]) as i32;
        let mut progress = -1i32;

        Some(RefCell::new(move || {
            progress += 1;
            println!("{}%", (progress * 100) / number_of_possibilities);
        }))
    };

    for transform in remaining[0].iter() {
        for column in 0..(1 + board.width - transform.width) {
            for row in 0..(1 + board.height - transform.height) {
                for v in tracker.iter() {
                    v.borrow_mut()();
                }

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
        shape_from_template(10, vec!["**.", ".**", "..*"]),  // 11
        shape_from_template(1, vec![".*.", "***", ".*."]),   // 12
    ].iter()
        .map(Piece::all_transforms)
        .collect();
    let mut board = create_board(12, 5);

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