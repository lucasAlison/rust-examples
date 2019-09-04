pub fn execute() {
    let x = 10;
    println!("x: {}", x);

    let (x, y) = (5, 6);
    println!("x: {}, y: {}", x, y);

    let mut z = 15;
    println!("Z is mutable: {}", z);

    z = 20;
    println!("Z is mutable: {}", z);
}