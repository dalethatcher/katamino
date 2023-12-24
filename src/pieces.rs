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

#[cfg(test)]
mod tests {
    use crate::pieces::shape_from_template;

    #[test]
    fn can_create_shape_from_template() {
        let piece = shape_from_template(vec![
            "*.*",
            "***"]);

        assert_eq!(2, piece.height);
        assert_eq!(3, piece.width);
        assert_eq!(vec![true, false, true, true, true, true], piece.shape)
    }
}