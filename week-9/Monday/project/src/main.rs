fn main() {
    let x = 5;            // ----------+-- 'b (Lifetime of x)
                          //           |
    let r = &x;           // --+-- 'a  | (Lifetime of r)
                          //   |       |
    println!("r: {r}");   //   |       |
                          // --+       |
}                         // ----------+