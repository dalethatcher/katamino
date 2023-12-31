use std::cell::RefCell;
use std::time::Instant;

use crate::board::{create_board, Board, Placement};
use crate::pieces::{shape_from_template, Piece};

mod board;
mod pieces;

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

fn find_solutions<'a>(
    top_level: bool,
    board: &mut Board<'a>,
    remaining: &'a [Vec<Piece>],
) -> Vec<Board<'a>> {
    // reduces runtime to 1/4-1/8 of previous, but specific for this puzzle
    if !board.empty_spaces_multiple_of_five() {
        #[cfg(feature = "trace")]
        {
            println!("Pruning impossible path:");
            print_state(board);
        }
        return vec![];
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

    let mut solutions = vec![];
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
                    if remaining.len() == 1 {
                        println!("Found solution:");
                        print_state(board);
                        solutions.push(board.clone());
                    } else {
                        let mut child_solutions = find_solutions(false, board, &remaining[1..]);
                        solutions.append(&mut child_solutions);
                    }
                    board.remove_last();
                }
            }
        }
    }

    solutions
}

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
        shape_from_template(10, vec!["**.", ".**", "..*"]),  // 11
        shape_from_template(1, vec![".*.", "***", ".*."]),   // 12
    ];

    #[cfg(feature = "trace")]
    {
        for piece in pieces.iter() {
            println!("piece {}:", piece.id);
            println!("{}", piece.shape_string());
        }
    }

    let transforms: Vec<Vec<Piece>> = pieces.iter().map(Piece::all_transforms).collect();
    let mut board = create_board(12, 5);

    let start = Instant::now();
    let solutions = find_solutions(true, &mut board, transforms.as_slice());

    if solutions.is_empty() {
        println!("no solution found :(");
    } else {
        let elapsed = start.elapsed();
        println!("found all solutions in {}ms!", elapsed.as_millis());
        print_state(&board);
    }
}

#[cfg(test)]
mod tests {
    use crate::board::create_board;
    use crate::find_solutions;
    use crate::pieces::{shape_from_template, Piece};

    #[test]
    fn can_place_pieces() {
        let pieces: Vec<Vec<Piece>> = vec![
            shape_from_template(1, vec!["*.*", "***"]),
            shape_from_template(2, vec!["*.*", "***"]),
            shape_from_template(3, vec![".*.", "***", ".*."]),
        ]
        .iter()
        .map(Piece::all_transforms)
        .collect();
        let mut board = create_board(5, 3);

        let solutions = find_solutions(true, &mut board, pieces.as_slice());
        assert_eq!(2, solutions.len());
    }
}
