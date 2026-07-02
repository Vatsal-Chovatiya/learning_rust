pub trait Summary {
    // Instead of ending with a semicolon, we provide a block of code
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

// An empty impl block means we are using the default summarize method
impl Summary for NewsArticle {}

fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    // We can still call summarize, even though we didn't write it for NewsArticle
    println!("New article available! {}", article.summarize());
}