use std::collections::HashSet;

#[derive(Clone)]
#[derive(PartialEq)]
pub(crate) struct Piece {
    pub(crate) height: u8,
    pub(crate) width: u8,
    pub(crate) shape: Vec<bool>,
}


pub(crate) fn shape_from_template(template: Vec<&str>) -> Piece {
    let mut height: u8 = 0;
    let mut width: u8 = 0;
    let mut shape: Vec<bool> = Vec::new();

    for line in template {
        height += 1;
        let current_width = u8::try_from(line.len()).ok().unwrap();

        if width == 0 {
            width = current_width;
        } else if current_width != width {
            panic!("on line {} got width {} but expected {}", height, current_width, width)
        }

        for c in line.chars() {
            shape.push(c == '*');
        }
    }

    Piece { height, width, shape }
}

impl Piece {
    pub(crate) fn flip_horizontaly(&self) -> Piece {
        let mut shape: Vec<bool> = Vec::with_capacity(self.shape.len());

        for r in 0..self.height {
            for c in 0..self.width {
                let copy_index = usize::from(r * self.width + (self.width - c - 1));

                shape.push(self.shape[copy_index]);
            }
        }

        Piece { shape, ..*self }
    }

    pub(crate) fn rotate_clockwise(&self) -> Piece {
        let mut shape: Vec<bool> = Vec::with_capacity(self.shape.len());

        for r in 0..self.width {
            for c in 0..self.height {
                let from_row = self.height - c - 1;
                let from_column = r;
                let copy_index = usize::from(from_row * self.width + from_column);

                shape.push(self.shape[copy_index]);
            }
        }

        Piece { height: self.width, width: self.height, shape }
    }

    pub(crate) fn shape_string(&self) -> String {
        let mut result = String::new();

        for i in 0..self.shape.len() {
            if i > 0 && i % usize::from(self.width) == 0 {
                result.push_str("\n");
            }
            result.push_str(if self.shape[i] { "█" } else { " " });
        }

        result
    }

    fn shape_id(&self) -> (u32, u8, u8) {
        let mut result: u32 = 0;

        for i in (0..self.shape.len()).rev() {
            result <<= 1;
            if self.shape[i] {
                result += 1;
            }
        }

        (result, self.width, self.height)
    }

    pub(crate) fn all_transforms(&self) -> Vec<Piece> {
        let mut result = vec![];
        let mut existing: HashSet<(u32, u8, u8)> = HashSet::new();

        let mut add_all_rotations = |piece: &Piece| {
            let mut current = piece.clone();

            for i in 0..4 {
                if i > 0 {
                    current = current.rotate_clockwise();
                }

                let shape_id = current.shape_id();
                if !existing.contains(&shape_id) {
                    existing.insert(shape_id);
                    result.push(current.clone());
                }
            }
        };

        add_all_rotations(self);

        let flipped = self.flip_horizontaly();
        add_all_rotations(&flipped);

        result
    }

    pub(crate) fn is_solid(&self, row: u8, column: u8) -> bool {
        return self.shape[usize::from(row * self.width + column)];
    }
}

#[cfg(test)]
mod tests {
    use crate::pieces::{shape_from_template};

    #[test]
    fn can_create_shape_from_template() {
        let piece = shape_from_template(vec![
            "*.*",
            "***"]);

        assert_eq!(2, piece.height);
        assert_eq!(3, piece.width);
        assert_eq!(vec![true, false, true, true, true, true], piece.shape);
    }

    #[test]
    fn can_flip_piece_horizontally() {
        let input = shape_from_template(vec![
            "*..",
            "***",
        ]);

        let flipped = input.flip_horizontaly();

        assert_eq!(2, flipped.height);
        assert_eq!(3, flipped.width);
        assert_eq!(vec![false, false, true, true, true, true], flipped.shape);
    }

    #[test]
    fn can_rotate_3x2_clockwise() {
        let input = shape_from_template(vec![
            "*..",
            "***",
        ]);

        let rotated = input.rotate_clockwise();

        assert_eq!(2, rotated.width);
        assert_eq!(3, rotated.height);
        assert_eq!(vec![true, true, true, false, true, false], rotated.shape);
    }

    #[test]
    fn can_rotate_2x2_clockwise() {
        let input = shape_from_template(vec![
            "**",
            "*.",
        ]);

        let rotated = input.rotate_clockwise();
        assert_eq!(2, rotated.width);
        assert_eq!(2, rotated.height);
        assert_eq!(vec![true, true, false, true], rotated.shape);
    }

    #[test]
    fn can_convert_to_shape_string() {
        let input = shape_from_template(vec![
            "*..",
            "***",
        ]);

        assert_eq!("█  \n███", input.shape_string());
    }

    #[test]
    fn can_calculate_shape_id() {
        let input = shape_from_template(vec![
            "**",
            ".*",
            "..",
        ]);

        assert_eq!((0xb, 2, 3), input.shape_id())
    }

    #[test]
    fn can_generate_all_transforms_for_1d() {
        let input = shape_from_template(vec!["**"]);
        let rotated_input = input.rotate_clockwise();

        let transforms = input.all_transforms();
        assert_eq!(2, transforms.len());
        assert!(transforms.contains(&input));
        assert!(transforms.contains(&rotated_input));
    }

    #[test]
    fn can_generate_all_transforms_for_2d() {
        let input = shape_from_template(vec!["**", "*.", "*."]);
        let transforms = input.all_transforms();

        assert_eq!(8, transforms.len());
    }

    #[test]
    fn can_test_solidity() {
        let input = shape_from_template(vec!["**", "*."]);

        assert!(input.is_solid(0, 0));
        assert!(input.is_solid(0, 1));
        assert!(input.is_solid(1, 0));
        assert!(!input.is_solid(1, 1));
    }
}