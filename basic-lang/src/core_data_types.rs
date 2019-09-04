use std::mem;

pub fn execute() {
    // is also immutable variable
    // 8 bits u => unsigned(0 or positive number), value can be 0..255
    let a:u8 = 255;
    println!("a = {}",a);

    // b is mutable because we use the keyword mut
    let mut b:i8 = 0;
    println!("b = {}",b);
    b=42;
    println!("b = {}",b);

    let c = 123456789;
    println!("c = {} size {} bytes",c,mem::size_of_val(&c));
}