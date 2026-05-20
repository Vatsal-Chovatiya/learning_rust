#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // Prints the math, then assigns 60 to width
        height: 50,
    };

    dbg!(&rect1); // Pass a reference so dbg! doesn't steal ownership
}