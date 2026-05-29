fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    v.push(6); // 1. Push first (memory might move here)

    let first = &v[0]; // 2. Get the reference *after* the move

    println!("The first element is: {first}"); // 3. Safe to use! None => println!("There is no third element."),
}
