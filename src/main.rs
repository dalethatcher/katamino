use crate::board::{Board, Placement};
use crate::pieces::{Piece, shape_from_template};

mod pieces;
mod board;

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
                    }
                    else {
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
        shape_from_template(vec![
            "*..",
            "***",
        ]),
        shape_from_template(vec!["**"]),
    ].iter()
        .map(Piece::all_transforms)
        .collect();
    let mut board = Board {
        width: 3,
        height: 2,
        placements: vec![],
    };

    if place_pieces(&mut board, shapes.as_slice()) {
        println!("found solution!");
        for row in board.index_grid() {
            for index in row {
                print!("{}", index);
            }
            println!();
        }
    } else {
        println!("no solution found :(");
    }
}
