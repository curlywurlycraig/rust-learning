#[derive(Debug, Clone)]
enum UsState {
    Alabama,
    Alaska,
    Ohio,
    California
}

#[derive(Debug, Clone)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(UsState::Ohio) => {
                println!("An ohio one!");
                25
            },
            Coin::Quarter(state) => {
                println!("Quarter for state {:?}", state);
                25
            }
        }
    }

    fn value_in_cents_from_option(coin: &Option<Coin>) -> Option<u8> {
        match coin {
            Some(inner_coin) => Some(inner_coin.value_in_cents()),
            None => None
        }
    }
}

trait Valuable {
    fn value_in_cents(&self) -> u8;
}

impl Valuable for Option<Coin> {
    fn value_in_cents(&self) -> u8 {
        match self {
            Some(inner_coin) => inner_coin.value_in_cents(),
            None => 0
        }
    }
}

fn main() {
    println!("A dime is worth {} cents", Coin::Dime.value_in_cents());
    println!("The value of an Alaskan Quarter is {}", Coin::Quarter(UsState::Alaska).value_in_cents());

    let option_coin = Some(Coin::Nickel);
    Coin::value_in_cents_from_option(&option_coin);
    println!("The value of a nickel is {}", option_coin.value_in_cents());

    println!("The value of nothing is {}", None.value_in_cents());

    let _ = Coin::Quarter(UsState::Ohio).value_in_cents();

    let thing = Some(Coin::Quarter(UsState::Ohio));

    if let Some(other_thing) = &thing {
        println!("Other thing is {:?}", other_thing);
    }

    match &thing {
        Some(yes) => Coin::Penny,
        Some(_) => Coin::Penny,
        _ => Coin::Penny
    };

    println!("Thing is {:?}", thing);
}
