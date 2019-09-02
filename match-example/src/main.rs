fn main() {
    let age: u8 = 27; // u8 is an 8-bit unsigned integer

    // match can return values ​​too
    let category = match age {
        // 0..=4 it's an inclusive standard (inclui 0, 1, 2, 3 e 4)
        0..=4 => "baby",
        0..=13 => "kid",
        14..=17 => "teen",
        _ => "adult"
    }; // ; required when using return

    println!("With {} year(s) you is considered {}.",age, category);
}
