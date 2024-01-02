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

    fn place_remaining_pieces(&mut self, remaining: &'a [Vec<Piece>]) -> Vec<Vec<Vec<i32>>> {
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
                            println!("Found solution:");
                            self.print_state();
                            solutions.push(self.piece_id_grid());
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

    pub(crate) fn find_solutions(
        &mut self,
        transforms: &'a Arc<Vec<Vec<Piece>>>,
    ) -> Vec<Vec<Vec<i32>>> {
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
                            println!("Found solution:");
                            self.print_state();
                            solutions.push(self.piece_id_grid());
                        } else {
                            let child_board_width = self.width;
                            let child_board_height = self.height;
                            let child_pieces = Arc::clone(&transforms);

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
}

#[cfg(test)]
mod tests {
    use crate::board::{create_board, Placement};
    use crate::pieces::{shape_from_template, Piece};
    use std::sync::Arc;

    #[test]
    fn can_add_to_empty_board() {
        let piece = shape_from_template(1, vec!["*.*", "***"]);
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
        let first_piece = shape_from_template(1, vec!["*****"]);
        let second_piece = shape_from_template(2, vec!["**..", ".***"]);
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
        let first_piece = shape_from_template(1, vec!["*****"]);
        let second_piece = shape_from_template(2, vec!["**..", ".***"]);

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
        let piece = shape_from_template(1, vec!["*****"]);
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
        let piece = shape_from_template(1, vec!["**..", ".***"]);
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
        let first_piece = shape_from_template(100, vec!["*****"]);
        let second_piece = shape_from_template(200, vec!["*****"]);
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
        let piece = shape_from_template(0, vec!["*****"]);
        let transforms = piece.all_transforms();
        let board = create_board(12, 5);

        assert_eq!(18, board.number_of_top_level_possibilities(&transforms))
    }

    #[test]
    fn can_check_empty_space_is_multiple_of_five() {
        let piece = shape_from_template(0, vec!["*****"]);
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
            shape_from_template(94, vec!["*****"]),              //  1
            shape_from_template(208, vec!["*...", "****"]),      //  2
            shape_from_template(130, vec![".*..", "****"]),      //  3
            shape_from_template(127, vec!["**..", ".***"]),      //  4
            shape_from_template(4, vec!["***", "..*", "..*"]),   //  5
            shape_from_template(217, vec!["***", "**."]),        //  6
            shape_from_template(11, vec!["***", "*.*"]),         //  7
            shape_from_template(6, vec!["**.", ".*.", ".**"]),   //  8
            shape_from_template(252, vec![".*.", "**.", ".**"]), //  9
            shape_from_template(28, vec!["*..", "***", "*.."]),  // 10
            shape_from_template(10, vec!["..*", ".**", "**."]),  // 11
            shape_from_template(1, vec![".*.", "***", ".*."]),   // 12
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
            shape_from_template(1, vec!["*.*", "***"]),
            shape_from_template(2, vec!["*.*", "***"]),
            shape_from_template(3, vec![".*.", "***", ".*."]),
            shape_from_template(4, vec!["*****"]),
        ]
        .iter()
        .map(Piece::all_transforms)
        .collect();
        let pieces = Arc::new(pieces);
        let mut board = create_board(5, 4);

        let solutions = board.find_solutions(&pieces);
        assert_eq!(1, solutions.len());
    }
}
