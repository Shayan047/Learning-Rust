enum Direction {
    Left,
    Right,
}

enum Color {
    Red,
    Blue,
    Orange,
    Black
}

fn color_name(name: Color) {
    match name {
        Color::Red => println!("Color is red"),
        Color::Blue => println!("Color is blue"),
        Color::Orange => println!("Color is orange"),
        Color::Black => println!("Color is black"),
    }
}

fn main() {
    let go = Direction::Left;

    match go {
        Direction::Left => println!("Go left"),
        Direction::Right => println!("Go right"),
    }

    color_name(Color::Blue);
}