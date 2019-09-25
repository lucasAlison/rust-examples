extern crate libc;

extern {
    fn sum(number: libc::c_int) -> libc::c_int;
}

fn main() {
    let number = 8;
    print!("{} + {} = {}",number,number,unsafe { sum(number) });
}