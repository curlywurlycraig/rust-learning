use std::io;
use std::collections::HashMap;

struct Item {
    description: String,
    identifier: String 
}

impl Item {
    fn new(identifier: &str, description: &str) -> Item {
        Item{ identifier: identifier.to_string(),
              description: description.to_string() }
    }
}

struct EnvState;

struct Position {
    directions: HashMap<String, Position>,
    description: String
}

impl Position {
    fn new(description: &str) -> Position {
        Position{ directions: HashMap::new(),
                  description: description.to_string() }
    }

    fn move_direction(&self, direction: &str) -> Result<&Position, String> {
        match self.directions.get(direction) {
            None => Err("Unable to change direction".to_string()),
            Some(result) => Ok(result)
        }
    }

    fn add_direction(&mut self, direction: &str, position: &Position) {
        self.directions.insert(direction.to_string(), position);
    }
}

struct GameState {
    inventory: Vec<Item>,
    environment: Vec<EnvState>,
    position: Position
}

fn main() {
    // initial position
    let mut position = Position::new("You wake up from an uncomfortable sleep on an uncomfortable chair. You appear to be hurtling through the air at 300 miles an hour. Ahead of you is a terrible flatscreen television with an array of movies, and to either side of you are complete strangers.");

    let corridor = Position::new("You are between rows in a long wide fuselage. You hear the occasional cough and whisper, but they're low and quiet under the constant drone of the engine.");

    position.add_direction("corridor", &corridor);

    let mut input = String::new();

    loop {
        // get the next commmand
        print!("{}\n> ", position.description);
        io::stdin().read_line(&mut input);

        // parse the command
        if input.starts_with("quit") {
            return;
        } else if input.starts_with("go ") {
            match position.move_direction(&input[3..]) {
                Ok(new_position) => position = *new_position,
                Err(message)     => println!("I don't know how to get there.")
            }
        }

        input.clear();
    }
}
