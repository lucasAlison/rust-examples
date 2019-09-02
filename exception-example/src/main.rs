use std::fs::File;
use std::io::Read;

fn main() {
    match File::open("./exemplo.txt") {
        Ok(mut file) => { // If file exist
            let mut txt = String::new();
            file.read_to_string(&mut txt).ok();
            println!("{}", txt);
        }
        Err(err) => println!("Erro: {}", err), // If file not exist
    }
}