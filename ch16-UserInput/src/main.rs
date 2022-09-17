use std::io;

fn main() {
    println!("Please type something, or x to escape.");
    let mut input_string = String::new();

    while input_string.trim() != "x" {
        input_string.clear();
        io::stdin()
            .read_line(&mut input_string)
            .expect("Failed to read line");
        println!("You typed: {}", input_string);
    }
}
