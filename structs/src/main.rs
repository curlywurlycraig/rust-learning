// Learnings:
// 1. Pretty printing of debugs using {:#?}
// 2. Use of the #[derive(x)] attribute
// 3. Structs in general.
// 4. Tuple structs (named tuples)
//
// Structs cannot have some mutable and some non-mutable attributes.

fn main() {
    let mut user1 = Player::new(String::from("craig"), String::from("craig@crwi.uk"));

    let user2 = Player {
        username: String::from("meg"),
        position: Point(10, 10),
        ..user1
    };

    let user3 = Player {
        username: String::from("collider"),
        position: Point(10, 10),
        ..user1
    };

    user1.move_position(Point(101, 202));

    println!("user1 and user2 collide? {}", user1.collides_with(&user2));
    println!("user2 and user3 collide? {}", user2.collides_with(&user3));
}

#[derive(Debug, Copy, Clone)]
struct Point(i32, i32);

struct Player {
    position: Point,
    username: String,
    sign_in_count: u64,
    active: bool
}

impl Player {
    // Not a particularly valuable function, since we could just modify position
    // directly, but a good example of &mut self
    fn move_position(&mut self, new_point: Point) {
        self.position = new_point;
    }

    fn new(username: String, email: String) -> Player {
        Player {
            username,
            position: Point(0, 0),
            sign_in_count: 1,
            active: true
        }
    }

    fn collides_with(&self, other: &Player) -> bool {
        self.position.0 == other.position.0 && self.position.1 == other.position.1
    }

    // Interestingly, an associated method doesn't have to relate technically to the struct at all.
    // This is true in other languages too, but worth thinking about.
    fn give_integer() -> i32 {
        3
    }
}