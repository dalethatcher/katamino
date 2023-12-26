use crate::pieces::shape_from_template;

mod pieces;

fn main() {
    let root_shape = shape_from_template(vec![
        "*..",
        "***",
    ]);

    let transforms = root_shape.all_transforms();
    println!("All transforms:");
    for piece in transforms {
        println!("---");
        println!("{}", piece.shape_string());
    }
    println!("---");
}
