extern crate bcrypt;

use bcrypt::{DEFAULT_COST, hash};
use std::time::Instant;

fn main() {
    let start = Instant::now();    
    let hashed = hash("1234", DEFAULT_COST).unwrap();
    let elapsed = start.elapsed();
    println!("Millis: {} ms", elapsed.as_millis());
    println!("{:?}", hashed);
}