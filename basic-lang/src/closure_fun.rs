pub fn execute() {
    let add_one = |i:i32| -> i32 { i + 1 };
    println!("Closure in Rust 1+1={}", add_one(1));
}