pub fn execute() {
    let hello = "Hello there!";

    let s:&'static str = "Hello there!"; // string slice, static alocated and glued into the program, we cannot change the value or re-asign, you cant do s[0]

    println!("{}", hello);
    println!("{}", s);
    println!("{}", s==hello);

    for c in s.chars() {
        println!("{}", c);
    }

    if let Some(first_char) = s.chars().nth(0) {
        println!("First char == {}", first_char);
    }
}