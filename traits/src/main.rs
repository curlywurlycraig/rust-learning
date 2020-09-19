// Important learning regarding &self, &mut self, and self in method signatures.
// let &item = something;
//
// destructures item and thus item is not a reference.
//
// but fn(&self) { ... }
//
// doesn't destructure, it is syntax sugar to specify that self is a &Self
// You can also do:
// fn(self: &Self)
// or:
// fn(self: &mut Self)
// or:
// fn(self: Self).
//
// The latter takes ownership of the thing whose method is called.

trait Thing {
    fn thing(&self) {}
}

trait Test: Thing {
    fn test(&self) -> u8 {
        self.thing();
        1
    }

    fn other_test(&mut self);
}

impl Test for Example {
    fn test(&self) -> u8 {
        1
    }

    fn other_test(&mut self) {
        self.item = 10;
    }
}

#[derive(Clone)]
struct Example {
    item: u8
}

impl Thing for Example {}

fn main() {
    let mut yes = Example {
        item: 5
    };

    yes.other_test();

    println!("holy cow! {}", yes.item);
}
