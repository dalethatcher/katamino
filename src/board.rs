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

    pub(crate) fn piece_id_grid(&self) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![vec![-1; usize::from(self.width)]; usize::from(self.height)];

        for placement in self.placements.iter() {
            for piece_row in 0..placement.piece.height {
                for piece_column in 0..placement.piece.width {
                    if placement.piece.is_solid(piece_row, piece_column) {
                        result[usize::from(placement.row + piece_row)]
                            [usize::from(placement.column + piece_column)] =
                            placement.piece.id;
                    }
                }
            }
        }

        result
    }

    pub(crate) fn contains_isolated_single(&self) -> bool {
        let u_width = usize::from(self.width);

        for (i, filled) in self.filled.iter().enumerate() {
            if !filled {
                let filled_above = i < u_width || self.filled[i - u_width];
                let filled_below = i >= self.filled.len() - u_width || self.filled[i + u_width];
                let filled_left = (i % u_width) == 0 || self.filled[i - 1];
                let filled_right = ((i + 1) % u_width) == 0 || self.filled[i + 1];

                if filled_above && filled_below && filled_right && filled_left {
                    return true;
                }
            }
        }

        false
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
    fn generates_expected_shape_id_grid() {
        let first_piece = shape_from_template(100, vec!["**", "*."]);
        let second_piece = shape_from_template(200, vec!["*"]);
        let mut board = create_board(2, 2);
        board.try_add(Placement { row: 0, column: 0, piece: &first_piece });
        board.try_add(Placement { row: 1, column: 1, piece: &second_piece });
        let shape_id_grid = board.piece_id_grid();

        assert_eq!(vec![vec![100, 100], vec![100, 200]], shape_id_grid);
    }

    macro_rules! isolated_test {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (width, height, template, expected) = $value;
                let piece = shape_from_template(1, template);
                let mut board = create_board(width, height);
                board.try_add(Placement{row: 0, column: 0, piece: &piece});

                assert_eq!(expected, board.contains_isolated_single());
            }
        )*
        }
    }

    isolated_test! {
        isolated_top_left: (2, 2, vec![".*", "**"], true),
        isolated_top_right: (2, 2, vec!["*.", "**"], true),
        isolated_bottom_right: (2, 2, vec!["**", "*."], true),
        isolated_bottom_left: (2, 2, vec!["**", ".*"], true),
        isolated_centre: (3, 3, vec!["***", "*.*", "***"], true),
        non_isolated: (2, 2, vec!["*"], false),
    }
}
