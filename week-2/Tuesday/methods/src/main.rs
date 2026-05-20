#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 1. Implementation block houses the methods
impl Rectangle {
    // 2. The first parameter must be self
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    // 3. Methods are called using dot notation
    println!("The area of the rectangle is {} square pixels.", rect1.area());
}