static mut COUNTER:i8 = 1;

pub fn execute() {
    unsafe {
        println!("Kids this is unsafe == {}", COUNTER);
    }
}