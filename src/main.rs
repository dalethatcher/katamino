use pieces::shape_from_template;
use crate::pieces::Piece;

mod pieces;

fn print_rotations(shape: &Piece) {
    let mut current_shape = Piece { height: shape.height, width: shape.width, shape: shape.shape.clone() };

    println!("{}", current_shape.to_string());
    for _ in 0..3 {
        println!("---");
        current_shape = current_shape.rotate_clockwise();
        println!("{}", current_shape.to_string());
    }
}

fn main() {
    let root_shape = shape_from_template(vec![
        "*..",
        "***",
    ]);

    print_rotations(&root_shape);
    println!("---");
    print_rotations(&root_shape.flip_horizontaly());
}
