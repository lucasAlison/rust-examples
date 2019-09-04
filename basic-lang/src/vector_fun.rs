pub fn execute() {
    // Vectors also allow for pop and push
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("Printing vectors in Rust> {:?}", v);

    for x in &v {
        println!("{}", x);
    }

    match v.get(3) {
        Some(x) => println!("v[3]=={}",x),
        None    => println!("No value for v[3]")
    }
}