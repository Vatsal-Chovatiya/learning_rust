// 1. Bring Asparagus into scope using a path shortcut
use crate::garden::vegetables::Asparagus;

// 2. Declare the garden module so the compiler looks for src/garden.rs
pub mod garden;

fn main() {
    // 3. Use the shortcut to create an instance
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
}



