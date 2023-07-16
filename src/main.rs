mod color;

use color::{Color, RGB};

fn main() {
    let test: RGB = RGB {
        red: 228,
        green: 228,
        blue: 228,
    };

    println!("{:?}", Color::rgb(test, false, 1, 1));

    // println!("{:?}", Color::index(4).id);
    // println!("{:?}", Color::index(4).name);
    // println!("{:?}", Color::index(4).red);
    // println!("{:?}", Color::index(4).green);
    // println!("{:?}", Color::index(4).blue);
    // println!("{:?}", Color::index(4).alpha);
}

struct Matrix {
    matrix: Vec<Vec<Option<u32>>>,
}
