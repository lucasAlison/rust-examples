fn main() {
    let website = Some("http://www.example.com");
    match website {
        None => println!("Website not specified."),
        Some(addr) => println!("Website: {}", addr),
    }

    let num: Option<i8> = Some(2); // i8 is a integer of 8 bits

    if num.is_none() {
        println!("Number not specified.");
    }

    if let Some(n) = num {
        println!("Number: {}", n);
    }
}
