use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // ZONE 1: The Main Attempt
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        
        // ZONE 2: The Backup Plan (If opening failed)
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|e| {
                panic!("Problem creating the file: {e:?}");
            })
        // ZONE 3: The Panic Button (If it's a weird error)
        } else {
            panic!("Problem opening the file: {error:?}");
        }

    });
}