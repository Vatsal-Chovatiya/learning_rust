pub trait Summary {
    // Instead of ending with a semicolon, we provide a block of code
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}