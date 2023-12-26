use crate::pieces::Piece;

#[derive(Clone)]
pub(crate) struct Placement {
    pub(crate) row: u8,
    pub(crate) column: u8,
    pub(crate) piece: Piece,
}

pub(crate) struct Board {
    pub(crate) width: u8,
    pub(crate) height: u8,
    pub(crate) placements: Vec<Placement>,
}

impl Board {
    pub(crate) fn empty(&self, row: u8, column: u8) -> bool {
        for placement in &self.placements {
            if row >= placement.row && row < placement.row + placement.piece.height &&
                column >= placement.column && column < placement.column + placement.piece.width &&
                placement.piece.is_solid(row - placement.row, column - placement.column)
            {
                return false;
            }
        }

        true
    }
    pub(crate) fn try_add(&mut self, placement: Placement) -> bool {
        for piece_column in 0..placement.piece.width {
            for piece_row in 0..placement.piece.height {
                if placement.piece.is_solid(piece_row, piece_column) {
                    if !self.empty(piece_row + placement.row, piece_column + placement.column) {
                        return false;
                    }
                }
            }
        }

        self.placements.push(placement.clone());
        true
    }
    pub(crate) fn remove_last(&mut self) {
        self.placements.pop();
    }
}

#[cfg(test)]
mod tests {
    use crate::board::{Board, Placement};
    use crate::pieces::shape_from_template;

    #[test]
    fn can_add_to_empty_board() {
        let piece = shape_from_template(vec!["*"]);
        let placement = Placement { row: 0, column: 0, piece: piece };
        let mut board = Board { width: 1, height: 1, placements: vec![] };

        assert!(board.try_add(placement));
    }

    #[test]
    fn cannot_add_to_full_board() {
        let first_piece = shape_from_template(vec!["**"]);
        let second_piece = shape_from_template(vec!["*"]);
        let mut board = Board { width: 2, height: 1, placements: vec![Placement { row: 0, column: 0, piece: first_piece }] };

        assert!(!board.try_add(Placement { row: 0, column: 1, piece: second_piece }));
    }

    #[test]
    fn can_add_after_removing() {
        let first_piece = shape_from_template(vec!["**"]);
        let second_piece = shape_from_template(vec!["*"]);

        let mut board = Board { width: 2, height: 1, placements: vec![Placement { row: 0, column: 0, piece: first_piece }] };
        board.remove_last();

        assert!(board.try_add(Placement { row: 0, column: 1, piece: second_piece }));
    }

    #[test]
    fn can_test_for_empty_out_of_bounds() {
        let piece = shape_from_template(vec!["*"]);
        let placement = Placement { row: 0, column: 0, piece: piece };
        let board = Board { width: 2, height: 2, placements: vec![placement] };

        assert!(!board.empty(0, 0));
        assert!(board.empty(0, 1));
        assert!(board.empty(1, 0));
        assert!(board.empty(1, 1));
    }

    #[test]
    fn can_test_for_empty_inside_bounds() {
        let piece = shape_from_template(vec!["**", "*."]);
        let placement = Placement { row: 0, column: 0, piece: piece };
        let board = Board { width: 2, height: 2, placements: vec![placement] };

        assert!(!board.empty(0, 0));
        assert!(!board.empty(0, 1));
        assert!(!board.empty(1, 0));
        assert!(board.empty(1, 1));
    }
}
