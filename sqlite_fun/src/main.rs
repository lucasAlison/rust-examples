fn main() {
    println!("SQL Lite + Rust");

    let connection = sqlite::open(":memory:").unwrap();
    connection
        .execute(
            "
                CREATE TABLE users (name TEXT, age INTEGER);
                INSERT INTO users VALUES ('Lucas', 22);
                INSERT INTO users VALUES ('Alison', 23);
            "
        )
    .unwrap();

    connection
    .iterate("SELECT * FROM users WHERE age > 22", |pairs| {
        for &(column, value) in pairs.iter() {
            println!("{} = {}", column, value.unwrap());
        }
        true
    })
    .unwrap();
}
