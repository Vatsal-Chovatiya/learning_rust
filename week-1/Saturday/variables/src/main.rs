fn main(){

    // Example of a mutable variable.
    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

    // Example of a constant variable.
    // Constants always in uppercase with underscore in between.
    const THREE_HOURS_IN_SECONDS:u32 = 60 * 60 * 3;
}