enum Color {
    Red(u8),
    Yellow(u8),
    Blue(u8),
    Orange,
    Purple,
    Green,
}

fn main() {
    let color = Color::Red(25);
    match color {
        Color::Red(b) | Color::Yellow(b) | Color::Blue(b) => print!("Primary color. Alpha: {}%", b),
        Color::Orange => println!("Orange."),
        _ => println!("too many colors."),
    }
}