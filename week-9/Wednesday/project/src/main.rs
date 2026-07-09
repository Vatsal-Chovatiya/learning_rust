fn main() {
    let ans;

    let str1 = String::from("Small");
    {
        let str2 = String::from("Longer");
        ans = longest(str1, str2);
    }
    println!("{:?}", ans)
}

fn longest(a: String, b: String) -> String {
    if a.len() > b.len() {
        return a;
    } else {
        return b;
    }
}
