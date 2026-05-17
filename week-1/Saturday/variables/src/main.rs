fn main() {
    // Example of a tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Can destructure the tuple or acess the value of it by using (.) operator.
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
