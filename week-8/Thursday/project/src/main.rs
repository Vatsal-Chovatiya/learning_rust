pub trait Summary {
    // This method has NO default implementation. It is strictly required.
    fn summarize_author(&self) -> String;

    // This method HAS a default implementation.
    // Notice how it calls the required summarize_author method!
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}


impl Summary for SocialPost {
    // We only write the required method
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    // summarize is automatically provided by the trait's default implementation
}

fn main() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    };

    // We call summarize. The default implementation runs, 
    // which internally calls our custom summarize_author.
    println!("1 new post: {}", post.summarize());
}