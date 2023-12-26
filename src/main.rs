use std::time::Instant;
use crate::board::{Board, Placement};
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

fn place_pieces(board: &mut Board, remaining: &[Vec<Piece>]) -> bool {
    for transform in &remaining[0] {
        for column in 0..(1 + board.width - transform.width) {
            for row in 0..(1 + board.height - transform.height) {
                let placement = Placement {
                    column,
                    row,
                    piece: transform.clone(),
                };

                if board.try_add(placement) {
                    if remaining.len() == 1 || place_pieces(board, &remaining[1..]) {
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
        // shape_from_template(vec![".*.", "***", ".*."]),       // 12
        // shape_from_template(vec!["*****"]),             //  1
        shape_from_template(vec!["*...", "****"]),      //  2
        shape_from_template(vec![".*..", "****"]),      //  3
        shape_from_template(vec!["**..", ".***"]),      //  4
        shape_from_template(vec!["*..", "*..", "***"]), //  5
        shape_from_template(vec!["***", "**."]),        //  6
        // shape_from_template(vec!["*.*", "***"]),        //  7
        shape_from_template(vec!["*..", "***", "..*"]), //  8
        // shape_from_template(vec!["*..", "***", ".*."]), //  9
        shape_from_template(vec!["***", ".*.", ".*."]), // 10
        shape_from_template(vec!["**.", ".**", "..*"]), // 11
    ].iter()
        .map(Piece::all_transforms)
        .collect();
    let mut board = Board {
        width: 8,
        height: 5,
        placements: vec![],
    };

    let start = Instant::now();
    if place_pieces(&mut board, shapes.as_slice()) {
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
    use crate::board::Board;
    use crate::pieces::{Piece, shape_from_template};

    #[test]
    fn can_place_pieces() {
        let pieces: Vec<Vec<Piece>> = vec![
            shape_from_template(vec!["*.*", "***"]),
            shape_from_template(vec!["*.*", "***"]),
            shape_from_template(vec![".*.", "***", ".*."]),
        ].iter().map(Piece::all_transforms).collect();
        let mut board = Board { width: 5, height: 3, placements: vec![] };

        assert!(place_pieces(&mut board, pieces.as_slice()));
        print_state(&board);
        assert!(true)
    }
}