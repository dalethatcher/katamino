use crate::pieces::Piece;

#[derive(Clone)]
pub(crate) struct Placement<'a> {
    pub(crate) row: u8,
    pub(crate) column: u8,
    pub(crate) piece: &'a Piece,
}

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
                    self.filled[usize::from((piece_row + placement.row) * self.width +
                        piece_column + placement.column)] = new_value;
                }
            }
        }
    }
    pub(crate) fn try_add(&mut self, placement: Placement<'a>) -> bool {
        for piece_column in 0..placement.piece.width {
            for piece_row in 0..placement.piece.height {
                if placement.piece.is_solid(piece_row, piece_column) {
                    if !self.empty(piece_row + placement.row, piece_column + placement.column) {
                        return false;
                    }
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

    pub(crate) fn index_grid(&self) -> Vec<Vec<i8>> {
        let mut result: Vec<Vec<i8>> = vec![vec![-1; usize::from(self.width)]; usize::from(self.height)];

        for (i, placement) in self.placements.iter().enumerate() {
            for piece_row in 0..placement.piece.height {
                for piece_column in 0..placement.piece.width {
                    if placement.piece.is_solid(piece_row, piece_column) {
                        result[usize::from(placement.row + piece_row)]
                            [usize::from(placement.column + piece_column)] =
                            i8::try_from(i).unwrap();
                    }
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::board::{create_board, Placement};
    use crate::pieces::shape_from_template;

    #[test]
    fn can_add_to_empty_board() {
        let piece = shape_from_template(1, vec!["*"]);
        let placement = Placement { row: 0, column: 0, piece: &piece };
        let mut board = create_board(1, 1);

        assert!(board.try_add(placement));
    }

    #[test]
    fn cannot_add_to_full_board() {
        let first_piece = shape_from_template(1, vec!["**"]);
        let second_piece = shape_from_template(2, vec!["*"]);
        let mut board = create_board(2, 1);
        board.try_add(Placement { row: 0, column: 0, piece: &first_piece });

        assert!(!board.try_add(Placement { row: 0, column: 1, piece: &second_piece }));
    }

    #[test]
    fn can_add_after_removing() {
        let first_piece = shape_from_template(1, vec!["**"]);
        let second_piece = shape_from_template(2, vec!["*"]);

        let mut board = create_board(2, 1);
        board.try_add(Placement { row: 0, column: 0, piece: &first_piece });
        board.remove_last();

        assert!(board.try_add(Placement { row: 0, column: 1, piece: &second_piece }));
    }

    #[test]
    fn can_test_for_empty_out_of_bounds() {
        let piece = shape_from_template(1, vec!["*"]);
        let mut board = create_board(2, 2);
        board.try_add(Placement { row: 0, column: 0, piece: &piece });

        assert!(!board.empty(0, 0));
        assert!(board.empty(0, 1));
        assert!(board.empty(1, 0));
        assert!(board.empty(1, 1));
    }

    #[test]
    fn can_test_for_empty_inside_bounds() {
        let piece = shape_from_template(1, vec!["**", "*."]);
        let mut board = create_board(2, 2);
        board.try_add(Placement { row: 0, column: 0, piece: &piece });

        assert!(!board.empty(0, 0));
        assert!(!board.empty(0, 1));
        assert!(!board.empty(1, 0));
        assert!(board.empty(1, 1));
    }

    #[test]
    fn generates_expected_index_grid() {
        let first_piece = shape_from_template(1, vec!["**", "*."]);
        let second_piece = shape_from_template(2, vec!["*"]);
        let mut board = create_board(2, 2);
        board.try_add(Placement { row: 0, column: 0, piece: &first_piece });
        board.try_add(Placement { row: 1, column: 1, piece: &second_piece });
        let index_grid = board.index_grid();

        assert_eq!(vec![vec![0, 0], vec![0, 1]], index_grid);
    }
}
