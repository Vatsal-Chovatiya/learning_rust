fn main() {
    let r;                // 1. Outer scope: declare 'r' without an initial value

    {                     // 2. Inner scope starts
        let x = 5;        // 3. Declare 'x' and give it the initial value of 5
        r = &x;           // 4. Attempt to set 'r' as a reference to 'x'
    }                     // 5. Inner scope ends: 'x' goes out of scope here

    println!("r: {r}");   // 6. Outer scope: attempt to print the value in 'r'
}