struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // Convert &str literals to String
    let mut user1 = build_user(
        String::from("someexample@gmail.com"),
        String::from("someone"),
    );
    
    // Silence the unused variable warning by doing something with it
    println!("User email: {}", user1.email);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
