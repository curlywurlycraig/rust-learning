use std::rc::Rc;
use std::collections::VecDeque;

struct Node {
    value: i32,
    children: Vec<Rc<Node>>,
}

impl Node {
    fn depth_first_iter(&self) -> NodeDepthFirstIter {
        NodeDepthFirstIter {
            search_nodes: VecDeque::from(vec![self])
        }
    }

    fn breadth_first_iter(&self) -> NodeBreadthFirstIter {
        NodeBreadthFirstIter {
            search_nodes: VecDeque::from(vec![self])
        }
    }
}

struct NodeDepthFirstIter<'a> {
    search_nodes: VecDeque<&'a Node>
}

impl<'a> Iterator for NodeDepthFirstIter<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.search_nodes.pop_front() {
            Some(node) => {
                for child in node.children.iter().rev() {
                    self.search_nodes.push_front(&child);
                }

                Some(node.value)
            },
            None => None
        }
    }
}

struct NodeBreadthFirstIter<'a> {
    search_nodes: VecDeque<&'a Node>
}

impl<'a> Iterator for NodeBreadthFirstIter<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.search_nodes.pop_front() {
            Some(node) => {
                for child in node.children.iter() {
                    self.search_nodes.push_back(&child);
                }

                Some(node.value)
            },
            None => None
        }
    }
}

fn main() {
    let a = Node {
        value: 0,
        children: vec![]
    };

    let b = Node {
        value: 1,
        children: vec![]
    };

    let c = Node {
        value: 2,
        children: vec![]
    };

    let d = Node {
        value: 3,
        children: vec![]
    };

    let ab = Node {
        value: 4,
        children: vec![Rc::new(a), Rc::new(b)]
    };

    let cd = Node {
        value: 5,
        children: vec![Rc::new(c), Rc::new(d)]
    };

    let root = Node {
        value: 6,
        children: vec![Rc::new(ab), Rc::new(cd)]
    };

    let depth: Vec<i32> = root.depth_first_iter().collect();
    assert_eq!(vec![6, 4, 0, 1, 5, 2, 3], depth);

    let breadth: Vec<i32> = root.breadth_first_iter().collect();
    assert_eq!(vec![6, 4, 5, 0, 1, 2, 3], breadth);

    let sum_from_depth: i32 = root.depth_first_iter().sum();
    let sum_from_breadth: i32 = root.breadth_first_iter().sum();
    assert_eq!(sum_from_breadth, sum_from_depth);
    assert_eq!(21, sum_from_depth);
}
