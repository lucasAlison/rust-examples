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
        Color::Red(b) | Color::Yellow(b) | Color::Blue(b) => print!("Cor primária. Alpha: {}%", b),
        Color::Orange => println!("Laranja."),
        _ => println!("Demais cores."),
    }
}
