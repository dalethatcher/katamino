use crate::pieces::Piece;
use std::sync::Arc;
use std::thread;

#[derive(Clone)]
pub(crate) struct Placement<'a> {
    pub(crate) row: u8,
    pub(crate) column: u8,
    pub(crate) piece: &'a Piece,
}

#[derive(Clone)]
pub(crate) struct Board<'a> {
    pub(crate) width: u8,
    pub(crate) height: u8,
    pub(crate) placements: Vec<Placement<'a>>,
    pub(crate) filled: Vec<bool>,
}

pub(crate) fn create_board<'a>(width: u8, height: u8) -> Board<'a> {
    Board {
        width,
        height,
        placements: vec![],
        filled: vec![false; usize::from(width * height)],
    }
}

impl<'a> Board<'a> {
    pub(crate) fn empty(&self, row: u8, column: u8) -> bool {
        !self.filled[usize::from(row * self.width + column)]
    }
    fn update_filled(&mut self, placement: &Placement, new_value: bool) {
        for piece_column in 0..placement.piece.width {
            for piece_row in 0..placement.piece.height {
                if placement.piece.is_solid(piece_row, piece_column) {
                    self.filled[usize::from(
                        (piece_row + placement.row) * self.width + piece_column + placement.column,
                    )] = new_value;
                }
            }
        }
    }
    pub(crate) fn try_add(&mut self, placement: Placement<'a>) -> bool {
        for piece_row in 0..placement.piece.height {
            for piece_column in 0..placement.piece.width {
                if placement.piece.is_solid(piece_row, piece_column)
                    && !self.empty(piece_row + placement.row, piece_column + placement.column)
                {
                    return false;
                }
            }
        }

        self.update_filled(&placement, true);
        self.placements.push(placement.clone());
        true
    }

    pub(crate) fn remove_last(&mut self) {
        let removed_placement = self.placements.pop().unwrap();

        self.update_filled(&removed_placement, false);
    }

    pub(crate) fn piece_id_grid(&self) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> =
            vec![vec![-1; usize::from(self.width)]; usize::from(self.height)];

        for placement in self.placements.iter() {
            for piece_row in 0..placement.piece.height {
                for piece_column in 0..placement.piece.width {
                    if placement.piece.is_solid(piece_row, piece_column) {
                        result[usize::from(placement.row + piece_row)]
                            [usize::from(placement.column + piece_column)] = placement.piece.id;
                    }
                }
            }
        }

        result
    }

    fn number_of_top_level_possibilities(&self, transforms: &[Piece]) -> u32 {
        transforms
            .iter()
            .map(|p| ((1 + (self.width - p.width) / 2) * (1 + (self.height - p.height) / 2)) as u32)
            .sum()
    }

    fn count_from(&self, visited: &mut [bool], row: u8, column: u8) -> u32 {
        let index = (row * self.width + column) as usize;
        if visited[index] || self.filled[index] {
            return 0;
        }

        let mut result = 1;
        visited[index] = true;

        if row > 0 {
            result += self.count_from(visited, row - 1, column);
        }
        if row < self.height - 1 {
            result += self.count_from(visited, row + 1, column);
        }
        if column > 0 {
            result += self.count_from(visited, row, column - 1);
        }
        if column < self.width - 1 {
            result += self.count_from(visited, row, column + 1);
        }

        result
    }

    pub(crate) fn empty_spaces_multiple_of_five(&self) -> bool {
        let mut visited = vec![false; self.width as usize * self.height as usize];

        for (i, filled) in self.filled.iter().enumerate() {
            if !filled
                && !visited[i]
                && self.count_from(
                    visited.as_mut_slice(),
                    i as u8 / self.width,
                    i as u8 % self.width,
                ) % 5
                    != 0
            {
                return false;
            }
        }

        true
    }

    pub(crate) fn print_state(&self) {
        for row in self.piece_id_grid() {
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

    fn found_solution(&self) -> String {
        let name_grid = self.name_grid();
        println!("Found solution: {}", name_grid);
        self.print_state();

        name_grid
    }

    fn place_remaining_pieces(&mut self, remaining: &'a [Vec<Piece>]) -> Vec<String> {
        if !self.empty_spaces_multiple_of_five() {
            #[cfg(feature = "trace")]
            {
                println!("Pruning impossible path:");
                self.print_state();
            }
            return vec![];
        }

        let mut solutions = vec![];
        for transform in remaining[0].iter() {
            for row in 0..(1 + self.height - transform.height) {
                for column in 0..(1 + self.width - transform.width) {
                    let placement = Placement {
                        column,
                        row,
                        piece: transform,
                    };

                    if self.try_add(placement) {
                        if remaining.len() == 1 {
                            solutions.push(self.found_solution());
                        } else {
                            let mut child_solutions = self.place_remaining_pieces(&remaining[1..]);
                            solutions.append(&mut child_solutions);
                        }
                        self.remove_last();
                    }
                }
            }
        }

        solutions
    }

    pub(crate) fn find_solutions(&mut self, transforms: &'a Arc<Vec<Vec<Piece>>>) -> Vec<String> {
        let mut output_progress = {
            let number_of_possibilities =
                self.number_of_top_level_possibilities(&transforms[0]) as i32;
            let mut progress = -1i32;
            move || {
                progress += 1;
                println!("{}%", (progress * 100) / number_of_possibilities);
            }
        };
        let mut solutions = vec![];
        let mut children = vec![];
        for (transform_index, transform) in transforms[0].iter().enumerate() {
            for column in 0..(1 + (self.width - transform.width) / 2) {
                for row in 0..(1 + (self.height - transform.height) / 2) {
                    let placement = Placement {
                        column,
                        row,
                        piece: transform,
                    };

                    if self.try_add(placement) {
                        if transforms.len() == 1 {
                            solutions.push(self.found_solution());
                        } else {
                            let child_board_width = self.width;
                            let child_board_height = self.height;
                            let child_pieces = Arc::clone(transforms);

                            let child_handle = thread::spawn(move || {
                                let child_placement = Placement {
                                    row,
                                    column,
                                    piece: &child_pieces[0][transform_index],
                                };
                                let mut child_board =
                                    create_board(child_board_width, child_board_height);
                                child_board.try_add(child_placement);

                                child_board.place_remaining_pieces(&child_pieces[1..])
                            });
                            children.push(child_handle);
                        }
                        self.remove_last();
                    }
                }
            }
        }

        for handle in children {
            let result = handle.join().unwrap();
            output_progress();

            solutions.extend_from_slice(&result);
        }
        output_progress();
        solutions
    }

    pub(crate) fn name_grid(&self) -> String {
        let mut buffer = vec![vec!['.'; self.width as usize]; self.height as usize];

        for placement in self.placements.iter() {
            let piece_name = placement.piece.name.name_char();

            for piece_row in 0..placement.piece.height {
                for piece_column in 0..placement.piece.width {
                    if placement.piece.is_solid(piece_row, piece_column) {
                        buffer[(placement.row + piece_row) as usize]
                            [(placement.column + piece_column) as usize] = piece_name
                    }
                }
            }
        }

        let rows: Vec<String> = buffer.into_iter().map(|c| c.iter().collect()).collect();

        rows.join(" ")
    }

    fn placement_to_bits(&self, placement: &Placement) -> u64 {
        let piece_max_row = placement.row + placement.piece.height - 1;
        let piece_max_column = placement.column + placement.piece.width - 1;
        let mut result = 0u64;

        for row in 0..self.height {
            for column in 0..self.width {
                result <<= 1;

                if row >= placement.row
                    && row <= piece_max_row
                    && column >= placement.column
                    && column <= piece_max_column
                    && placement
                        .piece
                        .is_solid(row - placement.row, column - placement.column)
                {
                    result += 1;
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::board::{create_board, Placement};
    use crate::pieces::{piece_from_name, PentominoName, Piece};
    use std::sync::Arc;

    #[test]
    fn can_add_to_empty_board() {
        let piece = piece_from_name(1, PentominoName::U);
        let placement = Placement {
            row: 0,
            column: 0,
            piece: &piece,
        };
        let mut board = create_board(5, 2);

        assert!(board.try_add(placement));
    }

    #[test]
    fn cannot_add_to_full_board() {
        let first_piece = piece_from_name(1, PentominoName::I);
        let second_piece = piece_from_name(2, PentominoName::N);
        let mut board = create_board(5, 2);
        board.try_add(Placement {
            row: 0,
            column: 0,
            piece: &first_piece,
        });

        assert!(!board.try_add(Placement {
            row: 0,
            column: 0,
            piece: &second_piece
        }));
    }

    #[test]
    fn can_add_after_removing() {
        let first_piece = piece_from_name(1, PentominoName::I);
        let second_piece = piece_from_name(2, PentominoName::N);

        let mut board = create_board(5, 3);
        board.try_add(Placement {
            row: 0,
            column: 0,
            piece: &first_piece,
        });
        board.remove_last();

        assert!(board.try_add(Placement {
            row: 0,
            column: 1,
            piece: &second_piece
        }));
    }

    #[test]
    fn can_test_for_empty_out_of_bounds() {
        let piece = piece_from_name(1, PentominoName::I);
        let mut board = create_board(5, 2);
        board.try_add(Placement {
            row: 0,
            column: 0,
            piece: &piece,
        });

        assert!(!board.empty(0, 0));
        assert!(board.empty(1, 0));
    }

    #[test]
    fn can_test_for_empty_inside_bounds() {
        let piece = piece_from_name(1, PentominoName::N);
        let mut board = create_board(4, 2);
        board.try_add(Placement {
            row: 0,
            column: 0,
            piece: &piece,
        });

        assert!(!board.empty(0, 0));
        assert!(board.empty(1, 0));
    }

    #[test]
    fn generates_expected_shape_id_grid() {
        let first_piece = piece_from_name(100, PentominoName::I);
        let second_piece = piece_from_name(200, PentominoName::I);
        let mut board = create_board(5, 2);
        board.try_add(Placement {
            row: 0,
            column: 0,
            piece: &first_piece,
        });
        board.try_add(Placement {
            row: 1,
            column: 0,
            piece: &second_piece,
        });
        let shape_id_grid = board.piece_id_grid();

        assert_eq!(
            vec![vec![100, 100, 100, 100, 100], vec![200, 200, 200, 200, 200]],
            shape_id_grid
        );
    }

    #[test]
    fn can_calculate_the_number_of_top_level_possibilities() {
        let piece = piece_from_name(0, PentominoName::I);
        let transforms = piece.all_transforms();
        let board = create_board(12, 5);

        assert_eq!(18, board.number_of_top_level_possibilities(&transforms))
    }

    #[test]
    fn can_check_empty_space_is_multiple_of_five() {
        let piece = piece_from_name(0, PentominoName::I);
        let mut board = create_board(12, 5);
        board.try_add(Placement {
            row: 0,
            column: 0,
            piece: &piece,
        });

        assert!(board.empty_spaces_multiple_of_five());
    }

    #[test]
    fn calculates_empty_space_correctly_for_known_good_solution() {
        let pieces = vec![
            piece_from_name(94, PentominoName::I),  //  1
            piece_from_name(208, PentominoName::L), //  2
            piece_from_name(130, PentominoName::Y), //  3
            piece_from_name(127, PentominoName::N), //  4
            piece_from_name(4, PentominoName::V)
                .rotate_clockwise()
                .rotate_clockwise(), //  5 vec!["***", "..*", "..*"]
            piece_from_name(217, PentominoName::P), //  6
            piece_from_name(11, PentominoName::U)
                .rotate_clockwise()
                .rotate_clockwise(), //  7 vec!["***", "*.*"]
            piece_from_name(6, PentominoName::Z)
                .flip_horizontaly()
                .rotate_clockwise(), //  8 vec!["**.", ".*.", ".**"]
            piece_from_name(252, PentominoName::F)
                .flip_horizontaly()
                .rotate_clockwise(), //  9 vec![".*.", "**.", ".**"]
            piece_from_name(28, PentominoName::T)
                .rotate_clockwise()
                .flip_horizontaly(), // 10 vec!["*..", "***", "*.."]
            piece_from_name(10, PentominoName::W).rotate_clockwise(), // 11 vec!["..*", ".**", "**."]
            piece_from_name(1, PentominoName::X),                     // 12
        ];
        let placements: Vec<Placement> = vec![
            (0, 0), //  1
            (3, 0), //  2
            (3, 5), //  3
            (3, 8), //  4
            (0, 9), //  5
            (1, 0), //  6
            (0, 6), //  7
            (1, 9), //  8
            (0, 4), //  9
            (1, 7), // 10
            (1, 1), // 11
            (2, 3), // 12
        ]
        .iter()
        .zip(pieces.iter())
        .map(|((r, c), piece)| Placement {
            row: *r,
            column: *c,
            piece: piece,
        })
        .collect();

        let mut board = create_board(12, 5);
        for (i, placement) in placements.iter().enumerate() {
            assert!(
                board.try_add(placement.clone()),
                "failed to add piece index {}",
                i
            );
            assert!(
                board.empty_spaces_multiple_of_five(),
                "expected valid empty space when adding piece index {}",
                i
            );
        }
    }

    #[test]
    fn can_find_unique_solutions() {
        let pieces: Vec<Vec<Piece>> = vec![
            piece_from_name(1, PentominoName::U),
            piece_from_name(2, PentominoName::U),
            piece_from_name(3, PentominoName::X),
            piece_from_name(4, PentominoName::I),
        ]
        .iter()
        .map(Piece::all_transforms)
        .collect();
        let pieces = Arc::new(pieces);
        let mut board = create_board(5, 4);

        let solutions = board.find_solutions(&pieces);
        assert_eq!(1, solutions.len());
    }

    #[test]
    fn generates_expected_name_grid() {
        let u_piece = piece_from_name(1, PentominoName::U);
        let x_piece = piece_from_name(2, PentominoName::X);
        let mut board = create_board(3, 4);

        assert!(board.try_add(Placement {
            row: 2,
            column: 0,
            piece: &u_piece,
        }));
        assert!(board.try_add(Placement {
            row: 0,
            column: 0,
            piece: &x_piece,
        }));
        assert_eq!(".X. XXX UXU UUU", board.name_grid());
    }

    #[test]
    fn generates_expected_placement_bits() {
        let piece = piece_from_name(1, PentominoName::U);
        let board = create_board(12, 5);
        let placement = Placement {
            row: 1,
            column: 1,
            piece: &piece,
        };

        assert_eq!(0x500700000000u64, board.placement_to_bits(&placement));
    }
}
