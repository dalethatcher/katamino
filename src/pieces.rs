struct Piece {
    height: u8,
    width: u8,
    shape: Vec<bool>,
}


fn shape_from_template(template: Vec<&str>) -> Piece {
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
    fn flip_horizontaly(&self) -> Piece {
        let mut shape: Vec<bool> = Vec::with_capacity(self.shape.len());

        for r in 0..self.height {
            for c in 0..self.width {
                let copy_index = usize::from(r * self.width + (self.width - c - 1));

                shape.push(self.shape[copy_index]);
            }
        }

        Piece { shape, ..*self }
    }

    fn rotate_clockwise(&self) -> Piece {
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
    fn can_rotate_clockwise() {
        let input = shape_from_template(vec![
            "*..",
            "***",
        ]);

        let rotated = input.rotate_clockwise();

        assert_eq!(2, rotated.width);
        assert_eq!(3, rotated.height);
        assert_eq!(vec![true, true, true, false, true, false], rotated.shape);
    }
}