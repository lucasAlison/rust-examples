use rand::Rng;

pub fn execute() {
    let mut rng = rand::thread_rng();
    let temperature:i16 = rng.gen_range(20, 100);

    if temperature <= 30 {
        println!("Super COLD. Temperature: {}", temperature);
    } else {
        println!("Super HOT. Temperature: {}", temperature);
    }

    println!("Today is very: {}",
    if temperature <= 3 {"COLD!"} else {"HOT!"});

    let result = if temperature <= 30 {"cold"} else {"hot"};
    println!("Today is: {}", result);
    
}