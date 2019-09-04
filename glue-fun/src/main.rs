extern crate glue;
use glue::prelude::*;

fn main(){
    if take(1.., is(alphabetic)).test("foobar") {
        println!("One or more alphabetic characters found!");
    }
}