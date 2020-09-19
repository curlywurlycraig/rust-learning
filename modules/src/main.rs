mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("Blackberries")
            }
        }
    }
}

fn main() {
    let mut meal = back_of_house::Breakfast::summer("Sourdough");

    println!("I'd like {} toast please", meal.toast);
}

