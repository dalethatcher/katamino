use std::collections::HashSet;

#[derive(Clone, PartialEq)]
pub(crate) struct Piece {
    pub(crate) id: i32,
    pub(crate) name: PentominoName,
    pub(crate) height: u8,
    pub(crate) width: u8,
    pub(crate) shape: Vec<bool>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub(crate) enum PentominoName {
    F,
    I,
    L,
    N,
    P,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

impl PentominoName {
    pub fn name_char(&self) -> char {
        use std::fmt::Write;

        let mut output = String::new();

        write!(&mut output, "{:?}", self).expect("Unexpected error while trying to write string");

        output.chars().next().unwrap()
    }
}
pub(crate) fn piece_from_name(id: i32, name: PentominoName) -> Piece {
    let template = match &name {
        PentominoName::F => vec!["*..", "***", ".*."],
        PentominoName::I => vec!["*****"],
        PentominoName::L => vec!["*...", "****"],
        PentominoName::N => vec!["**..", ".***"],
        PentominoName::P => vec!["***", "**."],
        PentominoName::T => vec!["***", ".*.", ".*."],
        PentominoName::U => vec!["*.*", "***"],
        PentominoName::V => vec!["*..", "*..", "***"],
        PentominoName::W => vec!["**.", ".**", "..*"],
        PentominoName::X => vec![".*.", "***", ".*."],
        PentominoName::Y => vec![".*..", "****"],
        PentominoName::Z => vec!["*..", "***", "..*"],
    };

    piece_from_template(id, name, template)
}

fn piece_from_template(id: i32, name: PentominoName, template: Vec<&str>) -> Piece {
    let mut height: u8 = 0;
    let mut width: u8 = 0;
    let mut shape: Vec<bool> = Vec::new();

    for line in template {
        height += 1;
        let current_width = u8::try_from(line.len()).ok().unwrap();

        if width == 0 {
            width = current_width;
        } else if current_width != width {
            panic!(
                "on line {} got width {} but expected {}",
                height, current_width, width
            )
        }

        for c in line.chars() {
            shape.push(c == '*');
        }
    }

    let square_count = shape.iter().filter(|s| **s).count();
    if square_count != 5 {
        panic!(
            "expected five squares but got {} for piece name {}",
            square_count,
            name.name_char()
        );
    }

    Piece {
        id,
        name,
        height,
        width,
        shape,
    }
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

        Piece {
            height: self.width,
            width: self.height,
            shape,
            ..*self
        }
    }

    #[cfg(feature = "trace")]
    pub(crate) fn shape_string(&self) -> String {
        let mut result = String::new();

        for i in 0..self.shape.len() {
            if i > 0 && i % usize::from(self.width) == 0 {
                result.push('\n');
            }
            result.push_str(if self.shape[i] { "█" } else { " " });
        }

        result
    }

    fn shape_id(&self) -> (u32, u8, u8) {
        (
            self.shape
                .iter()
                .fold(0u32, |acc, f| (acc << 1) + if *f { 1 } else { 0 }),
            self.width,
            self.height,
        )
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
        self.shape[usize::from(row * self.width + column)]
    }
}

#[cfg(test)]
mod tests {
    use crate::pieces::{piece_from_name, PentominoName};

    #[test]
    fn can_create_shape_from_template() {
        let piece = piece_from_name(123, PentominoName::U);

        assert_eq!(2, piece.height);
        assert_eq!(3, piece.width);
        assert_eq!(vec![true, false, true, true, true, true], piece.shape);
    }

    #[test]
    fn can_flip_piece_horizontally() {
        let input = piece_from_name(123, PentominoName::N);

        let flipped = input.flip_horizontaly();

        assert_eq!(123, flipped.id);
        assert_eq!(2, flipped.height);
        assert_eq!(4, flipped.width);
        assert_eq!(
            vec![false, false, true, true, true, true, true, false],
            flipped.shape
        );
    }

    #[test]
    fn can_rotate_3x2_clockwise() {
        let input = piece_from_name(123, PentominoName::U);

        let rotated = input.rotate_clockwise();

        assert_eq!(123, rotated.id);
        assert_eq!(2, rotated.width);
        assert_eq!(3, rotated.height);
        assert_eq!(vec![true, true, true, false, true, true], rotated.shape);
    }

    #[cfg(feature = "trace")]
    #[test]
    fn can_convert_to_shape_string() {
        let input = piece_from_name(123, PentominoName::L);

        assert_eq!("█  \n███", input.shape_string());
    }

    #[test]
    fn can_calculate_shape_id() {
        let input = piece_from_name(123, PentominoName::U);

        assert_eq!((0x2f, 3, 2), input.shape_id())
    }

    #[test]
    fn can_generate_all_transforms_for_1d() {
        let input = piece_from_name(123, PentominoName::I);
        let rotated_input = input.rotate_clockwise();

        let transforms = input.all_transforms();
        assert_eq!(2, transforms.len());
        assert!(transforms.contains(&input));
        assert!(transforms.contains(&rotated_input));
    }

    #[test]
    fn can_generate_all_transforms_for_2d() {
        let input = piece_from_name(123, PentominoName::F);
        let transforms = input.all_transforms();

        assert_eq!(8, transforms.len());
    }

    #[test]
    fn can_test_solidity() {
        let input = piece_from_name(123, PentominoName::P);

        assert!(input.is_solid(0, 0));
        assert!(input.is_solid(0, 2));
        assert!(input.is_solid(1, 0));
        assert!(!input.is_solid(1, 2));
    }

    #[test]
    fn get_expected_char_for_name() {
        assert_eq!('F', PentominoName::F.name_char());
    }
}
