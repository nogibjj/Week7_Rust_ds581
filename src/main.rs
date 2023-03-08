use std::io;
use week7_rust_ds581::is_valid;

fn main() {
    // Prompt the user to enter a string containing brackets
    println!("Please enter a string containing brackets:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Check whether the string is valid
    if is_valid(&input) {
        println!("The string is a valid string. Each open bracket has a corresponding close bracket");
    } else {
        println!("The string is either invalid or does not have an equal number of open and closed brackets.");
    }
}

