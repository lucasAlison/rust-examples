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
        Color::Red(a) | Color::Yellow(a) | Color::Blue(a) => {
            print!("Cor primária. Alpha: {}%", a);
        }
        Color::Orange => println!("Laranja."),
        _ => println!("Demais cores."),
    }
}
