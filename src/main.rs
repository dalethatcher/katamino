use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

use crate::board::create_board;
use crate::pieces::{piece_from_name, PentominoName, Piece};

mod board;
mod pieces;

fn canonicalise_solution_string(solution: &str) -> String {
    fn flip_horizontally(solution: &[String]) -> Vec<String> {
        solution.iter().map(|s| s.chars().rev().collect()).collect()
    }

    fn rotations(solution: &[String]) -> Vec<Vec<String>> {
        // currently only does a 180 flip as focusing on rectangle cases
        vec![solution
            .iter()
            .map(|s| s.chars().rev().collect())
            .rev()
            .collect()]
    }

    fn less_than(lhs: &[String], rhs: &[String]) -> bool {
        let l = lhs.iter().next().unwrap();
        let r = rhs.iter().next().unwrap();

        l < r
    }

    let input_solution: Vec<String> = solution.split_whitespace().map(|s| s.to_string()).collect();
    let input_rotations = rotations(&input_solution);
    let flipped_solution = flip_horizontally(&input_solution);
    let flipped_rotations = rotations(&flipped_solution);

    let mut minimum = &input_solution;
    for solution in input_rotations.iter() {
        if less_than(solution, minimum) {
            minimum = solution;
        }
    }
    if less_than(&flipped_solution, minimum) {
        minimum = &flipped_solution
    }
    for solution in flipped_rotations.iter() {
        if less_than(solution, minimum) {
            minimum = solution
        }
    }

    minimum.join(" ")
}

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
        let mut found = HashMap::new();
        for solution in solutions.iter() {
            let canonical_solution = canonicalise_solution_string(solution);

            match found.entry(canonical_solution) {
                Vacant(v) => {
                    v.insert(solution);
                }
                Occupied(o) => {
                    println!(
                        "discarding duplicate solution: {} duplicate of {}",
                        solution,
                        o.get()
                    );
                }
            }
        }

        println!(
            "found {} solutions with {} unique ones in {}ms!",
            solutions.len(),
            found.len(),
            elapsed.as_millis()
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::canonicalise_solution_string;

    #[test]
    fn generates_expected_canonical_string() {
        assert_eq!("AB CD", canonicalise_solution_string("AB CD"));
        assert_eq!("AB CD", canonicalise_solution_string("BA DC"));
        assert_eq!("AB CD", canonicalise_solution_string("DC BA"));
        assert_eq!("AB CD", canonicalise_solution_string("CD AB"));
        assert_eq!("ABC ADE", canonicalise_solution_string("ADE ABC"));
    }
}
