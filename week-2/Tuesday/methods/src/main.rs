// 1. Define the data structure
struct Player {
    name: String,
    level: u32,
}

// 2. Implementation block (the tool chest for Player)
impl Player {
    
    // --- ASSOCIATED FUNCTION (Constructor) ---
    // It doesn't use "self" because a specific player doesn't exist yet. 
    // It builds and returns a brand-new Player instance.
    fn new(username: &str) -> Self {
        Self {
            name: username.to_string(),
            level: 1, // Every new player starts at level 1
        }
    }

    // --- METHOD ---
    // It uses "&mut self" because it needs to modify a specific player's data.
    fn level_up(&mut self) {
        self.level += 1;
        println!("🎉 {} leveled up to Level {}!", self.name, self.level);
    }
}

// 3. Run the program
fn main() {
    // Call the Associated Function using :: syntax
    // This talks directly to the "Player" type to manufacture an instance.
    let mut gamer = Player::new("ShadowBlade");

    println!("Welcome {}, you are currently Level {}.", gamer.name, gamer.level);

    // Call the Method using . syntax
    // This tells our specific "gamer" instance to perform an action on itself.
    gamer.level_up();
}