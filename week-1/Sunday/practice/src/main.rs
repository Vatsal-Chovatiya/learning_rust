use std::io;

fn main() {
    let mut option = String::new();

    println!(
        "Press 1 to convert from Celsius to Fahrenheit and 2 to convert from Fahrenheit to Celsius:"
    );

    io::stdin()
        .read_line(&mut option)
        .expect("Failed to read line");

    let trimmed = option.trim();

    if trimmed == "1" || trimmed == "2" {
        println!("Success! You selected option: {}", trimmed);

        if trimmed == "1" {
            println!("Converting Celsius to Fahrenheit...");
        } else {
            println!("Converting Fahrenheit to Celsius...");
        }
    } else {
        println!(
            "Error: Invalid option '{}'. Please enter only 1 or 2.",
            trimmed
        );
    }
}
