#[derive(Debug)]
struct List {
    node: Option<i32>,
    rest: Option<Box<List>>
}

impl List {
    fn from(vec: &mut Vec<i32>) -> List {
        if vec.len() == 0 {
            return List {
                node: None,
                rest: None
            };
        }

        List {
            node: Some(vec.pop().expect("Failed to pop from Vec")),
            rest: Some(Box::new(List::from(vec)))
        }
    }

    fn iter(&self) -> ListIter {
        ListIter {
            list: self
        }
    }
}

struct ListIter<'a> {
    list: &'a List
}

impl<'a> Iterator for ListIter<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.list.node {
            Some(node) => {
                self.list = &self.list.rest.as_ref().expect("Failed to get the rest.");
                return Some(node);
            },
            None => {
                return None;
            }
        };
    }
}

fn main() {
    let list = List::from(&mut vec![1,2,3]);
    println!("Linked list is {:?}", list);

    for i in list.iter() {
        println!("Val {}", i);
    }

    for i in list.iter() {
        println!("I say it again: {}", i);
    }
}
