mod back_of_house {
    pub struct Breakfast {
        pub toast: String,      // Public: Customers can choose this
        seasonal_fruit: String, // Private: The chef chooses this secretly
    }

    impl Breakfast {
        // Because there is a private field, we MUST provide a public constructor function
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // 1. Order breakfast using the public constructor function
    let mut meal = back_of_house::Breakfast::summer("Rye");
    
    // 2. We can read and modify the public 'toast' field
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // 3. COMPILER ERROR: The next line will not compile if uncommented:
    // meal.seasonal_fruit = String::from("blueberries");
}